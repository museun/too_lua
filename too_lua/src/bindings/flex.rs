use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, LuaType, Mapping,
};

crate::make_struct! {
    struct FlexParams is "FlexParams" {
        /// Tight constraint (ratio between 0.0 and 1.0)
        tight = Option<f32> ; "number?"
        /// Loose constraint (ratio between 0.0 and 1.0)
        loose = Option<f32> ; "number?"
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Flex;

impl BindingView for Flex {
    const SPEC: BindingSpec = binding! {
        /// Give a flex constraint to its children
        "flex" => FlexParams::NAME
    };

    type Params = FlexParams;
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
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
