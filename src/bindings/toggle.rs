use too::view::Ui;

use crate::{
    mapping::{Binding, Field},
    params::Value,
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Toggle;

impl Toggle {
    binding! {
        /// Conditionally show or hide a view
        "toggle" => "toggle" {
            /// The boolean state to use
            value "Value"
        }
    }

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(value) = ctx.value() else {
            return Mapping::report_missing_data(ui, ctx.id, "show", "value");
        };

        let Value::Bool(value) = *value else {
            return Mapping::report_missing(ui, ctx.id, "bool value");
        };

        ui.toggle(value, |ui| ctx.visit_children(mapping, ui));
    }
}
