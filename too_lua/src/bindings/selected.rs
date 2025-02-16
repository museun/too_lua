use too::view::{Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    params::{self, Value},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Selected;

impl Selected {
    binding! {
        /// A selected boolean value
        "selected" => "selected" {
          /// The style of the selected value
          style "SelectedStyle?"
          /// The class of the selected value
          class "Selected?"
          /// The text of the selected value
          text "string"
          /// The state of the selected value, a boolean
          value "Value"
        }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<params::SelectedParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "selected", "params");
        };

        let Some(Ok(text)) = ctx.params_field::<String>("text") else {
            return Mapping::report_missing_data(ui, ctx.id, "selected", "text");
        };

        let Some(mut value) = ctx.value_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "selected", "value");
        };

        let Value::Bool(value) = &mut *value else {
            return Mapping::report_missing(ui, ctx.id, "bool value");
        };

        ui.show(too::views::selected(value, text).class(params.apply()));
    }
}
