use std::collections::{HashMap, VecDeque};

use mlua::{FromLua, UserData};
use too::helpers::hash_fnv_1a;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct LuaId(usize);

impl std::fmt::Debug for LuaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct Node {
    pub(crate) name: u64,
    #[allow(dead_code)]
    pub(crate) parent: Option<LuaId>,
    pub(crate) children: Vec<LuaId>,
    pub(crate) data: mlua::Value,
}

impl Node {
    fn new(name: mlua::String, parent: Option<LuaId>) -> Self {
        Self {
            parent,
            children: Vec::new(),
            name: hash_fnv_1a(&name.as_bytes()),
            data: mlua::Value::Nil,
        }
    }
}

impl std::ops::Index<LuaId> for Vec<Node> {
    type Output = Node;
    fn index(&self, LuaId(index): LuaId) -> &Self::Output {
        &self[index]
    }
}

impl std::ops::IndexMut<LuaId> for Vec<Node> {
    fn index_mut(&mut self, LuaId(index): LuaId) -> &mut Self::Output {
        &mut self[index]
    }
}

#[derive(Debug)]
pub struct Tree {
    pub(crate) root: LuaId,
    pub(crate) map: Vec<Node>,
    pub(crate) names: HashMap<LuaId, mlua::String>,
    pub(crate) lazies: HashMap<LuaId, mlua::Function>,

    stack: Vec<LuaId>,
    proxy: mlua::Table,
}

impl Tree {
    #[profiling::function]
    pub fn new(lua: &mlua::Lua) -> mlua::Result<Self> {
        // TODO cache these
        let mt = lua.create_table()?;
        mt.set("__call", Self::evaluate(lua)?)?;

        let table = lua.create_table()?;
        table.set_metatable(Some(mt));

        let mut map = vec![];

        let root_name = lua.create_string("__root__")?;
        // TODO end cache those

        let root = LuaId(map.len());
        map.push(Node::new(root_name.clone(), None));

        let mut names = HashMap::new();
        names.insert(root, root_name);

        Ok(Self {
            root,

            map,
            names,
            lazies: HashMap::new(),

            stack: vec![root],
            proxy: table,
        })
    }

    fn current_id(this: &Tree) -> LuaId {
        LuaId(this.map.len())
    }
}

impl Tree {
    #[profiling::function]
    pub(super) fn evaluate_lazies(&mut self) {
        for (&k, v) in &self.lazies {
            match &mut self.map[k].data {
                mlua::Value::Table(table) => match table.get::<mlua::String>("text") {
                    Ok(text) => {
                        let Ok(data) = v.call::<mlua::String>(()) else {
                            continue;
                        };
                        if text != data {
                            let _ = table.set("text", data);
                        }
                    }
                    _ => {
                        let Ok(data) = v.call::<mlua::Value>(()) else {
                            continue;
                        };

                        let key = match data {
                            mlua::Value::Boolean(_)
                            | mlua::Value::Integer(_)
                            | mlua::Value::Number(_) => "value",
                            mlua::Value::String(_) => "text",
                            _ => {
                                self.map[k].data = data;
                                return;
                            }
                        };

                        let _ = table.set(key, data);
                    }
                },

                mlua::Value::UserData(..) => {
                    let Ok(data) = v.call::<mlua::Value>(()) else {
                        continue;
                    };
                    self.map[k].data = data;
                }

                this @ mlua::Value::Nil => {
                    let Ok(data) = v.call::<mlua::Value>(()) else {
                        continue;
                    };
                    *this = data;
                }

                mlua::Value::String(string) => {
                    let Ok(data) = v.call::<mlua::String>(()) else {
                        continue;
                    };
                    if *string == data {
                        continue;
                    }
                    self.map[k].data = mlua::Value::String(data);
                }

                mlua::Value::Boolean(bool) => {
                    let Ok(data) = v.call::<bool>(()) else {
                        continue;
                    };
                    if *bool == data {
                        continue;
                    }
                    self.map[k].data = mlua::Value::Boolean(data);
                }

                mlua::Value::Integer(integer) => {
                    let Ok(data) = v.call::<mlua::Integer>(()) else {
                        continue;
                    };
                    if *integer == data {
                        continue;
                    }
                    self.map[k].data = mlua::Value::Integer(data);
                }

                mlua::Value::Number(float) => {
                    let Ok(data) = v.call::<mlua::Number>(()) else {
                        continue;
                    };
                    if *float == data {
                        continue;
                    }
                    self.map[k].data = mlua::Value::Number(data);
                }

                _ => {}
            }
        }
    }

    pub(super) fn add_lazy(&mut self, function: mlua::Function) {
        let current = *self.stack.last().unwrap_or(&self.root);
        self.lazies.insert(current, function);
    }

    fn proxy(&mut self, name: mlua::String) -> mlua::Result<mlua::Value> {
        let pid = self.stack.last().copied();

        let id = Self::current_id(self);
        self.map.push(Node::new(name.clone(), pid));
        self.names.insert(id, name);

        if let Some(parent) = pid {
            self.map[parent].children.push(id);
        }

        self.stack.push(id);
        Ok(mlua::Value::Table(self.proxy.clone()))
    }

    fn evaluate(lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
        lua.create_function(
            |lua, (_, args): (mlua::Value, mlua::Variadic<mlua::Value>)| {
                use mlua::Value as V;

                let id = lua
                    .app_data_mut::<Self>()
                    .expect("tree")
                    .stack
                    .pop()
                    .expect("valid shape");

                match &args.as_slice() {
                    [data @ V::Table(..)] => {
                        return Self::process(lua, id, data);
                    }
                    [data] => {
                        let mut tree = lua.app_data_mut::<Self>().expect("tree");
                        tree.map[id].data = data.clone();
                    }
                    _ => {}
                };

                Ok(V::Nil)
            },
        )
    }

    fn process(lua: &mlua::Lua, id: LuaId, value: &mlua::Value) -> mlua::Result<mlua::Value> {
        let mut queue = VecDeque::from_iter([value.clone()]);

        while let Some(item) = queue.pop_front() {
            match item {
                mlua::Value::Table(table) => {
                    let len = queue.len();

                    for (key, value) in table.pairs::<mlua::Value, mlua::Value>().flatten() {
                        if value.is_thread() {
                            table.set(key, mlua::Value::Nil)?;
                            queue.push_back(value);
                            continue;
                        }

                        // TODO find a better way of doing this
                        let is_empty = value.as_table().filter(|c| c.is_empty()).is_some();
                        if value.is_nil() || value.is_null() || is_empty {
                            table.set(key, mlua::Value::Nil)?;
                        }
                    }

                    if len != queue.len() {
                        let mut tree = lua.app_data_mut::<Self>().expect("tree");
                        tree.stack.push(id);
                        continue;
                    }

                    if !table.is_empty() {
                        let mut tree = lua.app_data_mut::<Self>().expect("tree");
                        tree.map[id].data = mlua::Value::Table(table)
                    }
                }

                mlua::Value::Thread(thread) => {
                    while thread.resume::<()>(Some(UiBuilder)).is_ok() {}
                }
                _ => {}
            }
        }

        let proxy = lua.app_data_ref::<Self>().unwrap().proxy.clone();
        Ok(mlua::Value::Table(proxy))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct UiBuilder;

impl FromLua for UiBuilder {
    fn from_lua(_: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        Ok(Self)
    }
}

impl UserData for UiBuilder {
    fn add_methods<M>(methods: &mut M)
    where
        M: mlua::UserDataMethods<Self>,
    {
        methods.add_meta_function("__index", |lua, (_, key): (Self, mlua::String)| {
            let Some(mut tree) = lua.app_data_mut::<Tree>() else {
                unreachable!("tree should be in the lua state")
            };
            tree.proxy(key)
        });
    }
}

#[derive(Debug)]
pub struct DebugNode {
    pub id: LuaId,
    pub name: String,
    pub data: mlua::Value,
    pub children: Vec<Self>,
}

impl DebugNode {
    pub fn build(tree: &Tree) -> Self {
        fn build(nodes: &mut Vec<DebugNode>, id: LuaId, tree: &Tree) {
            let mut children = vec![];
            let node = &tree.map[id];
            for &child in &node.children {
                build(&mut children, child, tree)
            }
            let node = DebugNode {
                id,
                name: tree.names[&id].to_string_lossy(),
                data: node.data.clone(),
                children,
            };
            nodes.push(node);
        }

        let mut children = vec![];
        let node = &tree.map[tree.root];
        for &node in &node.children {
            build(&mut children, node, tree);
        }

        Self {
            id: tree.root,
            name: tree.names[&tree.root].to_string_lossy(),
            data: node.data.clone(),
            children,
        }
    }

    pub fn print(&self, out: &mut impl std::io::Write) {
        fn print(children: &[DebugNode], prefix: &str, out: &mut impl std::io::Write) {
            for (i, node) in children.iter().enumerate() {
                let last = i == children.len() - 1;
                let upper = if last { "└─ " } else { "├─ " };

                _ = writeln!(
                    out,
                    "{prefix}{upper}{name}({id:?}): {data:?}",
                    name = node.name,
                    id = node.id,
                    data = node.data
                );

                let prefix = if last {
                    format!("{prefix}  ")
                } else {
                    format!("{prefix}| ")
                };

                print(&node.children, &prefix, out)
            }
        }

        _ = writeln!(
            out,
            "root({id:?}): {data:?}",
            id = self.id,
            data = self.data
        );
        print(&self.children, "", out);
    }
}
