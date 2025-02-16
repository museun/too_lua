#[macro_use]
mod macros;

mod extract;
use std::borrow::Cow;

use extract::merge;

mod align;
pub use align::Align;

mod axis;
pub use axis::Axis;

mod aligned;
pub use aligned::Aligned;

mod justify;
pub use justify::Justify;

mod cross_align;
pub use cross_align::CrossAlign;

mod progress;
pub use progress::{ProgressClass, ProgressParams, ProgressStyle};

mod slider;
pub use slider::{SliderClass, SliderParams, SliderStyle};

mod label;
pub use label::{LabelClass, LabelParams, LabelStyle};

mod button;
pub use button::{ButtonClass, ButtonParams, ButtonStyle};

mod border;
pub use border::{Border, BorderClass, BorderParams, BorderStyle};

mod checkbox;
pub use checkbox::{CheckboxClass, CheckboxParams, CheckboxStyle};

mod selected;
pub use selected::{SelectedClass, SelectedParams, SelectedStyle};

mod todo;
pub use todo::{TodoClass, TodoParams, TodoStyle};

mod toggle;
pub use toggle::{ToggleClass, ToggleParams, ToggleStyle};

mod constrained;
pub use constrained::{Constrained, Constraint};

mod color;
pub use color::Color;

mod value;
pub use value::Value;

use crate::Mapping;

trait Proxy: mlua::UserData + 'static {
    const NAME: &'static str;
    const KIND: Kind;

    // TODO figure out a better place for this
    const STYLE: Option<fn() -> &'static [(&'static str, &'static str, &'static str)]>;

    fn create(lua: &mlua::Lua) -> mlua::Result<()> {
        lua.globals().set(Self::NAME, lua.create_proxy::<Self>()?)
    }

    fn lua_bindings() -> &'static [(&'static str, &'static str)];
}

const fn proxy<T: Proxy>() -> ProxyObject {
    ProxyObject {
        kind: T::KIND,
        name: T::NAME,
        create: T::create,
        bindings: T::lua_bindings,
        style: T::STYLE,
    }
}

struct ProxyObject {
    kind: Kind,
    name: &'static str,
    create: fn(&mlua::Lua) -> mlua::Result<()>,
    bindings: fn() -> &'static [(&'static str, &'static str)],
    style: Option<fn() -> &'static [(&'static str, &'static str, &'static str)]>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Kind {
    Value,
    Enum,
}

const PROXY_OBJECTS: &[ProxyObject] = &[
    // values
    proxy::<Value>(),
    proxy::<Constrained>(),
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
    proxy::<Aligned>(),
    proxy::<Axis>(),
    proxy::<Border>(),
    proxy::<CrossAlign>(),
    proxy::<Justify>(),
];

pub fn initialize(lua: &mlua::Lua) -> mlua::Result<()> {
    PROXY_OBJECTS
        .iter()
        .try_for_each(|proxy| (proxy.create)(lua))
}

pub fn generate() -> String {
    use std::fmt::Write as _;
    let mut out = String::new();

    _ = writeln!(
        &mut out,
        "---@alias Color string #RGB | #RGBA | #RRGGBB | #RRGGBBAA hex string"
    );
    _ = writeln!(&mut out);

    for object in PROXY_OBJECTS {
        match object.kind {
            Kind::Value => {
                _ = writeln!(&mut out, "---@class (exact) {}", object.name);
                let bindings = (object.bindings)();
                for (binding, doc) in bindings {
                    _ = writeln!(&mut out, "---@field {binding} {doc}");
                }
                _ = writeln!(&mut out, "{} = {{}}", object.name);
                _ = writeln!(&mut out);
            }
            Kind::Enum => {
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

    for (.., binding) in Mapping::DEFAULT_TOO_BINDINGS {
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
    for (.., binding) in Mapping::DEFAULT_TOO_BINDINGS {
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
