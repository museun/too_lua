use std::collections::VecDeque;

use mlua::{FromLua, UserData};
use slotmap::{Key, SecondaryMap, SlotMap};
use too::helpers::hash_fnv_1a;

slotmap::new_key_type! {
    pub struct LuaId;
}

#[derive(Debug)]
pub struct Node {
    pub(crate) name: u64,
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

pub struct Tree {
    pub(crate) root: LuaId,
    pub(crate) map: SlotMap<LuaId, Node>,
    pub(crate) names: SecondaryMap<LuaId, String>,

    stack: Vec<LuaId>,
    proxy: mlua::Table,
}

impl Tree {
    pub fn new(lua: &mlua::Lua) -> mlua::Result<Self> {
        let mt = lua.create_table()?;
        mt.set("__call", Self::evaluate(lua)?)?;

        let table = lua.create_table()?;
        table.set_metatable(Some(mt));

        let mut map = SlotMap::with_key();

        let root_name = lua.create_string("__root__")?;
        let root = map.insert(Node::new(root_name.clone(), None));

        let mut names = SecondaryMap::new();
        names.insert(root, root_name.to_string_lossy());

        Ok(Self {
            map,
            stack: vec![root],
            root,
            proxy: table,
            names,
        })
    }
}

impl Tree {
    fn proxy(&mut self, name: mlua::String) -> mlua::Result<mlua::Value> {
        let pid = self.stack.last().copied();
        let id = self.map.insert(Node::new(name.clone(), pid));

        self.names.insert(id, name.to_string_lossy());
        if self.root.is_null() {
            self.root = id
        }

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
                        lua.app_data_mut::<Self>().expect("tree").map[id].data = data.clone();
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
                        lua.app_data_mut::<Self>().expect("tree").stack.push(id);
                        continue;
                    }

                    if !table.is_empty() {
                        lua.app_data_mut::<Self>().expect("tree").map[id].data =
                            mlua::Value::Table(table)
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

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct DebugSlotMap<'a>(&'a SlotMap<LuaId, Node>, &'a SecondaryMap<LuaId, String>);

        impl std::fmt::Debug for DebugSlotMap<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                struct Resolved<'a>(&'a Node, &'a str);
                impl std::fmt::Debug for Resolved<'_> {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.debug_struct("Node")
                            .field("name", &self.1)
                            .field("parent", &self.0.parent.map(|c| c.data()))
                            .field("children", &self.0.children)
                            .field("data", &self.0.data)
                            .finish()
                    }
                }

                let mut map = f.debug_map();
                for (k, v) in self.0 {
                    map.entry(&k.data(), &Resolved(v, &self.1[k]));
                }
                map.finish()
            }
        }

        f.debug_struct("Tree")
            .field("map", &DebugSlotMap(&self.map, &self.names))
            .field("stack", &self.stack)
            .field("root", &self.root.data())
            .field("proxy", &self.proxy)
            .finish()
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
