use mlua::AnyUserData;
use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, Mapping,
};

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
        "toggle" => "ToggleParams"
    };

    type Params = ToggleParams;
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.foo::<ToggleParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle", "params");
        };

        let Some(value) = ctx.value_ref(&params.value) else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle", "value");
        };

        let Some(value) = value.bool_ref() else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle", "bool");
        };

        ui.toggle(*value, |ui| ctx.visit_children(mapping, ui));
    }
}
