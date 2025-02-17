use std::{borrow::Cow, collections::BTreeSet};

#[macro_use]
mod macros;

use crate::{
    bindings::{
        Align, AlignedParams, Axis, BorderClass, BorderKind, ButtonClass, CheckboxClass,
        Constraint, CrossAlign, Justify, LabelClass, ProgressClass, SelectedClass, SliderClass,
        TodoClass, ToggleClass, Value,
    },
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
        proxy::<ToggleClass>(),
        // enums
        proxy::<Align>(),
        proxy::<AlignedParams>(),
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

pub trait Proxy: mlua::UserData + 'static {
    const NAME: &'static str;
    const KIND: ProxyKind;

    // TODO figure out a better place for this
    const STYLE: Option<fn() -> &'static [(&'static str, &'static str, &'static str)]>;

    fn create(lua: &mlua::Lua) -> mlua::Result<()> {
        lua.globals().set(Self::NAME, lua.create_proxy::<Self>()?)
    }

    fn lua_bindings() -> &'static [(&'static str, &'static str)];
}

pub const fn proxy<T: Proxy>() -> ProxyObject {
    ProxyObject {
        kind: T::KIND,
        name: T::NAME,
        create: T::create,
        bindings: T::lua_bindings,
        style: T::STYLE,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProxyObject {
    kind: ProxyKind,
    name: &'static str,
    create: fn(&mlua::Lua) -> mlua::Result<()>,
    bindings: fn() -> &'static [(&'static str, &'static str)],
    // TODO redo this
    style: Option<fn() -> &'static [(&'static str, &'static str, &'static str)]>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProxyKind {
    Value,
    Enum,
}

pub(crate) fn initialize<'i>(
    proxies: impl IntoIterator<Item = &'i ProxyObject>,
    lua: &mlua::Lua,
) -> mlua::Result<()> {
    proxies
        .into_iter()
        .try_for_each(|proxy| (proxy.create)(lua))
}

pub fn generate<'a, 'b>(proxies: &Proxies, bindings: &Bindings) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();

    _ = writeln!(
        &mut out,
        "---@alias Color string #RGB | #RGBA | #RRGGBB | #RRGGBBAA hex string"
    );
    _ = writeln!(&mut out);

    for object in proxies {
        match object.kind {
            ProxyKind::Value => {
                _ = writeln!(&mut out, "---@class (exact) {}", object.name);
                let bindings = (object.bindings)();
                for (binding, doc) in bindings {
                    _ = writeln!(&mut out, "---@field {binding} {doc}");
                }
                _ = writeln!(&mut out, "{} = {{}}", object.name);
                _ = writeln!(&mut out);
            }
            ProxyKind::Enum => {
                if let Some(style) = object.style {
                    _ = writeln!(&mut out, "---@class (exact) {}Style", object.name);
                    for (field, ty, doc) in (style)() {
                        _ = writeln!(&mut out, "---@field {field} {ty} {doc}")
                    }
                    _ = writeln!(&mut out, "{}Style = {{}}", object.name);
                    _ = writeln!(&mut out);
                }

                _ = writeln!(&mut out, "---@enum {}", object.name);
                _ = writeln!(&mut out, "{} = {{", object.name);
                let bindings = (object.bindings)();
                let padding = bindings.iter().fold(0, |max, (s, _)| max.max(s.len()));
                for (i, (binding, doc)) in bindings.iter().enumerate() {
                    _ = writeln!(&mut out, "    --- {doc}");
                    _ = writeln!(
                        &mut out,
                        "    {binding}{sp:padding$} = {i},",
                        sp = "",
                        padding = padding - binding.len()
                    );
                }
                _ = writeln!(&mut out, "}}");
                _ = writeln!(&mut out);
            }
        }
    }

    let bindings = bindings.into_iter().collect::<Vec<_>>();
    for (binding, ..) in &bindings {
        _ = writeln!(
            &mut out,
            "---@class {name} {doc}",
            name = binding.name,
            doc = binding.doc
        );
        for field in binding.fields {
            _ = writeln!(
                &mut out,
                "---@field {name} {ty} {doc}",
                name = field.name,
                ty = field.ty,
                doc = field.doc
            );
        }
        _ = writeln!(&mut out);
    }

    _ = writeln!(&mut out, "---@class ui");
    for (binding, ..) in bindings {
        let args = match binding.args {
            Some(args) => Cow::Owned(format!("args: {args}")),
            None => Cow::Borrowed(""),
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
