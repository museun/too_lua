use too::view::Ui;

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Flex;

impl Flex {
    binding! {
        /// Give a flex constraint to its children
        "flex" => "flex" {
            /// Tight constraint (ratio between 0.0 and 1.0)
            tight "number?"
            /// Loose constraint (ratio between 0.0 and 1.0)
            loose "number?"
        }
    }

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        use too::layout::Flex;
        if let Some(Ok(factor)) = ctx.params_field::<f32>("tight") {
            ui.show_children(too::views::Flexible::new(Flex::Tight(factor)), |ui| {
                ctx.visit_children(mapping, ui)
            });

            return;
        };

        let factor = ctx.params_field_opt("loose").unwrap_or(1.0);
        ui.show_children(too::views::Flexible::new(Flex::Loose(factor)), |ui| {
            ctx.visit_children(mapping, ui)
        });
    }
}
