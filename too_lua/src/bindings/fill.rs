use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, Mapping,
};

use super::Color;

crate::make_struct! {
    struct FillParams is "FillParams" {
        /// Use this color to fill the area
        background = Color       ; "string"
        /// Optional width to allocate
        width      = Option<u16> ; "integer?"
        /// Optional height to allocate
        height     = Option<u16> ; "integer?"
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Fill;

impl BindingView for Fill {
    const SPEC: BindingSpec = binding! {
        /// Fill the childrens area, with an optional size constraint
        "fill" => "FillParams"
    };

    type Params = FillParams;
    type Style = ();

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        use too::views::Fill;
        let Some(params) = ctx.params::<FillParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "fill", "params");
        };

        match (params.width, params.height) {
            (Some(w), Some(h)) => {
                ui.show(Fill::new(params.background, (w as i32, h as i32)));
            }
            _ => {
                ui.show(Fill::fill_with(params.background));
            }
        }
    }
}
