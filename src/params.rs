#[macro_use]
mod macros;

mod extract;
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

// TODO this should produce a lua-ls annotation file
trait Proxy: mlua::UserData + 'static {
    fn create(lua: &mlua::Lua) -> mlua::Result<()>;
}

const PROXY_OBJECTS: &[fn(&mlua::Lua) -> mlua::Result<()>] = &[
    // values
    Value::create,
    Constrained::create,
    // class
    ProgressClass::create,
    SliderClass::create,
    BorderClass::create,
    LabelClass::create,
    ButtonClass::create,
    CheckboxClass::create,
    SelectedClass::create,
    TodoClass::create,
    ToggleClass::create,
    // enums
    Border::create,
    Aligned::create,
    Align::create,
    Axis::create,
    Justify::create,
    CrossAlign::create,
];

pub fn initialize(lua: &mlua::Lua) -> mlua::Result<()> {
    PROXY_OBJECTS.into_iter().try_for_each(|f| f(lua))
}
