use too::view::{Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    params::{self, Value},
    Context, Mapping,
};

pub struct Checkbox;

impl Checkbox {
    binding! {
        /// A checkbox to toggle a boolean
        "checkbox" => "checkbox" {
            /// The style of the checkbox
            style "CheckboxStyle?"
            /// The class of the checkbox
            class "Checkbox?"
            /// The text of the checkbox
            text "string"
            /// The state of the checkbox, a boolean
            value "Value"
        }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<params::CheckboxParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "checkbox", "params");
        };

        let Some(Ok(text)) = ctx.params_field::<String>("text") else {
            return Mapping::report_missing_data(ui, ctx.id, "checkbox", "text");
        };

        let Some(mut value) = ctx.value_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "checkbox", "value");
        };

        let Value::Bool(value) = &mut *value else {
            return Mapping::report_missing(ui, ctx.id, "bool value");
        };

        ui.show(too::views::checkbox(value, text).class(params.apply()));
    }
}
