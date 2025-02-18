use std::{
    borrow::Cow,
    collections::{BTreeSet, HashSet},
};

#[macro_use]
mod macros;
pub use macros::{MergeStyle, Params, TranslateClass};

use crate::{
    bindings::{
        Align, AlignedKind, Axis, BorderClass, BorderKind, ButtonClass, CheckboxClass, Constraint,
        CrossAlign, Justify, LabelClass, ProgressClass, SelectedClass, SliderClass, TodoClass,
        ToggleSwitchClass, Value,
    },
    mapping::BindingArgs,
    Bindings,
};

#[derive(Default)]
pub struct Proxies {
    set: BTreeSet<ProxyObject>,
}

impl<'a> IntoIterator for &'a Proxies {
    type Item = &'a ProxyObject;
    type IntoIter = std::collections::btree_set::Iter<'a, ProxyObject>;
    fn into_iter(self) -> Self::IntoIter {
        self.set.iter()
    }
}

impl Proxies {
    const DEFAULT_PROXY_OBJECTS: &[ProxyObject] = &[
        // values
        proxy::<Value>(),
        proxy::<Constraint>(),
        // class
        proxy::<BorderClass>(),
        proxy::<ButtonClass>(),
        proxy::<CheckboxClass>(),
        proxy::<LabelClass>(),
        proxy::<ProgressClass>(),
        proxy::<SelectedClass>(),
        proxy::<SliderClass>(),
        proxy::<TodoClass>(),
        proxy::<ToggleSwitchClass>(),
        // enums
        proxy::<Align>(),
        proxy::<AlignedKind>(),
        proxy::<Axis>(),
        proxy::<BorderKind>(),
        proxy::<CrossAlign>(),
        proxy::<Justify>(),
    ];

    pub fn default_proxies() -> Self {
        Self::default().with_many(Self::DEFAULT_PROXY_OBJECTS.iter().copied())
    }

    pub fn with_many(self, many: impl IntoIterator<Item = ProxyObject>) -> Self {
        many.into_iter().fold(self, |this, proxy| this.with(proxy))
    }

    pub fn with(mut self, proxy: ProxyObject) -> Self {
        self.set.insert(proxy);
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LuaFunction {
    pub name: &'static str,
    pub doc: &'static str,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LuaField {
    pub name: &'static str,
    pub doc: &'static str,
    pub ty: &'static str,
}

pub trait LuaType: 'static {
    const NAME: &'static str;
    const KIND: ProxyKind;

    fn lua_fields() -> &'static [LuaField] {
        &[]
    }

    fn lua_functions() -> &'static [LuaFunction] {
        &[]
    }
}

impl LuaType for () {
    const NAME: &'static str = "";
    const KIND: ProxyKind = ProxyKind::Ignore;
}

pub trait Proxy: mlua::UserData + LuaType {
    fn create(lua: &mlua::Lua) -> mlua::Result<()> {
        lua.globals().set(Self::NAME, lua.create_proxy::<Self>()?)
    }
}

pub const fn proxy<T: Proxy>() -> ProxyObject {
    ProxyObject {
        kind: T::KIND,
        name: T::NAME,
        create: T::create,
        functions: T::lua_functions,
        fields: T::lua_fields,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProxyObject {
    kind: ProxyKind,
    name: &'static str,
    create: fn(&mlua::Lua) -> mlua::Result<()>,
    functions: fn() -> &'static [LuaFunction],
    fields: fn() -> &'static [LuaField],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProxyKind {
    Value,
    Enum,
    Args,
    Ignore,
}

pub(crate) fn initialize(proxies: &Proxies, lua: &mlua::Lua) -> mlua::Result<()> {
    proxies
        .into_iter()
        .try_for_each(|proxy| (proxy.create)(lua))
}

pub fn generate(proxies: &Proxies, bindings: &Bindings) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();

    _ = writeln!(
        &mut out,
        "---@alias Color string #RGB | #RGBA | #RRGGBB | #RRGGBBAA hex string"
    );
    _ = writeln!(&mut out);

    _ = writeln!(
        &mut out,
        "---@type fun(func: table<fun(): string>) lazily generate a string\n\
        ---@diagnostic disable-next-line: lowercase-global\n\
        lazy = function(func) end\n\
        ---@alias lazy_args nil"
    );
    _ = writeln!(&mut out);

    _ = writeln!(&mut out, "---@class rt");
    _ = writeln!(&mut out, "rt = {{");
    for &crate::runtime::Method {
        kind,
        name,
        args,
        params,
        returns,
        doc,
    } in crate::runtime::Runtime::lua_bindings()
    {
        _ = writeln!(&mut out, "    --- {doc}");
        if let crate::runtime::Kind::Async = kind {
            _ = writeln!(&mut out, "    ---@async");
        }
        _ = writeln!(&mut out, "    ---@params {args}");
        _ = writeln!(
            &mut out,
            "    ---@return {}",
            if returns.is_empty() { "nil" } else { returns }
        );
        _ = writeln!(&mut out, "    {name} = function({params}) end,");
    }
    _ = writeln!(&mut out, "}}");

    _ = writeln!(&mut out);

    for object in proxies {
        match object.kind {
            ProxyKind::Value => {
                _ = writeln!(&mut out, "---@class (exact) {}", object.name);
                let bindings = (object.functions)();
                for LuaFunction { name, doc } in bindings {
                    _ = writeln!(&mut out, "---@field {name} {doc}");
                }
                _ = writeln!(&mut out, "{} = {{}}", object.name);
                _ = writeln!(&mut out);
            }

            ProxyKind::Enum => {
                let styles = (object.fields)();
                if !styles.is_empty() {
                    _ = writeln!(&mut out, "---@class (exact) {}Style", object.name);
                    for LuaField { name, ty, doc } in styles {
                        _ = writeln!(&mut out, "---@field {name} {ty} {doc}")
                    }
                    _ = writeln!(&mut out, "{}Style = {{}}", object.name);
                    _ = writeln!(&mut out);
                }

                _ = writeln!(&mut out, "---@enum {}", object.name);
                _ = writeln!(&mut out, "{} = {{", object.name);
                let bindings = (object.functions)();
                let padding = bindings
                    .iter()
                    .fold(0, |max, LuaFunction { name, .. }| max.max(name.len()));

                for (i, LuaFunction { name, doc }) in bindings.iter().enumerate() {
                    _ = writeln!(&mut out, "    --- {doc}");
                    _ = writeln!(
                        &mut out,
                        "    {name}{sp:padding$} = {i},",
                        sp = "",
                        padding = padding - name.len()
                    );
                }
                _ = writeln!(&mut out, "}}");
                _ = writeln!(&mut out);
            }

            ProxyKind::Args => {
                let styles = (object.fields)();
                if !styles.is_empty() {
                    _ = writeln!(&mut out, "---@class (exact) {}Args", object.name);
                    for LuaField { name, ty, doc } in styles {
                        _ = writeln!(&mut out, "---@field {name} {ty} {doc}")
                    }
                    _ = writeln!(&mut out, "{}Args = {{}}", object.name);
                    _ = writeln!(&mut out);
                }
            }

            ProxyKind::Ignore => {}
        }
    }

    let mut seen_params = HashSet::new();
    // params for views
    for (binding, ..) in bindings {
        let BindingArgs::Named(name) = binding.args else {
            continue;
        };

        if !seen_params.insert(name) {
            continue;
        }

        _ = writeln!(&mut out, "---@class {name} {doc}", doc = binding.doc);
        for field in (binding.fields)() {
            _ = writeln!(
                &mut out,
                "---@field {name} {ty} {doc}",
                name = field.name,
                ty = field.ty,
                doc = field.doc
            );
        }
        _ = writeln!(&mut out, "{name } = {{}}");
        _ = writeln!(&mut out);

        if !binding.params.name.is_empty() && seen_params.insert(binding.params.name) {
            _ = writeln!(&mut out, "---@class {name}", name = binding.params.name);
            for field in binding.params.fields {
                _ = writeln!(
                    &mut out,
                    "---@field {name} {ty} {doc}",
                    name = field.name,
                    ty = field.ty,
                    doc = field.doc
                );
            }
            _ = writeln!(&mut out, "{name } = {{}}", name = binding.params.name);
            _ = writeln!(&mut out);
        }
    }

    _ = writeln!(&mut out, "---@class ui");
    for (binding, ..) in bindings {
        let args = match binding.args {
            BindingArgs::Named(args) => Cow::Owned(format!("args: {args}")),
            BindingArgs::Any => Cow::Borrowed("args: any"),
            BindingArgs::None => Cow::Borrowed(""),
        };

        _ = writeln!(
            &mut out,
            "---@field {name} fun({args}): nil {docs}",
            name = binding.name,
            docs = binding.doc
        );
    }
    _ = writeln!(&mut out, "ui = {{ }}");
    _ = writeln!(&mut out);

    out
}
