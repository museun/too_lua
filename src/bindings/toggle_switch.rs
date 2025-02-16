use too::view::{Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    params::{self, Value},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ToggleSwitch;

impl ToggleSwitch {
    binding! {
        /// A switch that is toggled when clicked
        "toggle_switch" => "toggle_switch" {
            /// The style of the selected value
            style "ToggleStyle?"
            /// The class of the selected value
            class "Toggle?"
            /// The state of the selected value, a boolean
            value "Value"
        }
    }

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<params::ToggleParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle", "params");
        };

        let Some(mut value) = ctx.value_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle", "value");
        };

        let Value::Bool(value) = &mut *value else {
            return Mapping::report_missing(ui, ctx.id, "bool value");
        };

        let axis = ctx.axis();

        ui.show(
            too::views::toggle_switch(value)
                .axis(axis)
                .class(params.apply()),
        );
    }
}
