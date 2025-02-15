#![cfg_attr(debug_assertions, allow(dead_code, unused_variables,))]

pub use make_class as make_enum;

mod application;
pub use application::Application;

mod script;
pub use script::Script;

mod mapping;
pub use mapping::{Context, Indirect, Mapping};

mod tree;
use tree::{LuaId, Node, UiBuilder};

pub use tree::Tree;

mod value;
pub use value::Value;

mod errors;
use errors::Errors;

mod notifications;
use notifications::{Notification, Notifications};

pub mod params;

fn setup_proxies(lua: &mlua::Lua) -> mlua::Result<()> {
    for (k, v) in [
        ("Value", lua.create_proxy::<value::Value>()?),
        ("Constraint", lua.create_proxy::<params::Constrained>()?),
        //
        ("Progress", lua.create_proxy::<params::ProgressClass>()?),
        ("Slider", lua.create_proxy::<params::SliderClass>()?),
        ("Border", lua.create_proxy::<params::BorderClass>()?),
        ("Label", lua.create_proxy::<params::LabelClass>()?),
        ("Button", lua.create_proxy::<params::ButtonClass>()?),
        ("Checkbox", lua.create_proxy::<params::CheckboxClass>()?),
        ("Selected", lua.create_proxy::<params::SelectedClass>()?),
        ("Todo", lua.create_proxy::<params::TodoClass>()?),
        ("Toggle", lua.create_proxy::<params::ToggleClass>()?),
        //
        ("BorderStyle", lua.create_proxy::<params::Border>()?),
        ("Aligned", lua.create_proxy::<params::Aligned>()?),
        ("Align", lua.create_proxy::<params::Align>()?),
        ("Axis", lua.create_proxy::<params::Axis>()?),
        ("Justify", lua.create_proxy::<params::Justify>()?),
        ("CrossAlign", lua.create_proxy::<params::CrossAlign>()?),
    ] {
        lua.globals().set(k, v)?;
    }
    Ok(())
}
