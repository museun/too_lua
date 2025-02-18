use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, Mapping,
};

use super::Color;

crate::make_struct! {
    struct BackgroundParams is "BackgroundParams" {
        /// The background color for the children
        background = Color ; "string"
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Background;

impl BindingView for Background {
    const SPEC: BindingSpec = binding! {
        /// Applies a background color to this children
        "background" => "BackgroundParams"
    };

    type Params = BackgroundParams;
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.foo::<BackgroundParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "background", "params");
        };

        ui.background(params.background, |ui| ctx.visit_children(mapping, ui));
    }
}
