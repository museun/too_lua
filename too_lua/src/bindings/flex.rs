use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, Mapping,
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
        "flex" => "FlexParams"
    };

    type Params = FlexParams;
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        use too::{layout::Flex, views::Flexible};

        let Some(params) = ctx.params::<FlexParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "flex", "params");
        };

        if let Some(flex) = params
            .tight
            .map(Flex::Tight)
            .or_else(|| params.loose.map(Flex::Loose))
        {
            ui.show_children(Flexible::new(flex), |ui| ctx.visit_children(mapping, ui));
            return;
        };

        Mapping::report_missing_data(ui, ctx.id, "flex", "params")
    }
}
