use too::view::Ui;

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

use super::Color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Background;

impl Background {
    binding! {
        /// Background of its children
        "background" => "background" {
            /// The background color for the children
            background "string"
        }
    }

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(Ok(bg)) = ctx.params_field::<Color>("background") else {
            return Mapping::report_missing_data(ui, ctx.id, "background", "bg");
        };

        ui.background(bg.0, |ui| ctx.visit_children(mapping, ui));
    }
}
