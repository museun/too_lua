use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, LuaType, Mapping,
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
        "background" => BackgroundParams::NAME
    };

    type Params = BackgroundParams;
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(Ok(bg)) = ctx.params_field::<Color>("background") else {
            return Mapping::report_missing_data(ui, ctx.id, "background", "bg");
        };

        ui.background(bg.0, |ui| ctx.visit_children(mapping, ui));
    }
}
