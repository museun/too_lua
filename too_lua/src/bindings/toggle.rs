use mlua::AnyUserData;
use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, LuaType, Mapping,
};

use super::Value;

make_struct! {
    struct ToggleParams is "ToggleParams" {
        /// The boolean state to use
        value = AnyUserData ; "Value"
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Toggle;

impl BindingView for Toggle {
    const SPEC: BindingSpec = binding! {
        /// Conditionally show or hide a view
        "toggle" => ToggleParams::NAME
    };

    type Params = ToggleParams;
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(value) = ctx.value() else {
            return Mapping::report_missing_data(ui, ctx.id, "show", "value");
        };

        let Value::Bool(value) = *value else {
            return Mapping::report_missing(ui, ctx.id, "bool value");
        };

        ui.toggle(value, |ui| ctx.visit_children(mapping, ui));
    }
}
