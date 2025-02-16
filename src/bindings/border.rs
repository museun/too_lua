use too::view::{Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    params, Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Border;

impl Border {
    binding! {
        /// Border to surround its children
        "border" => "border" {
            /// The style of the border
            style "BorderStyle?"
            /// The class of the border
            class "Border?"
            /// The border to use
            border "BorderKind"
        }
    }

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<params::BorderParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "border", "params");
        };

        let Some(Ok(border)) = ctx.params_field::<params::Border>("border") else {
            return Mapping::report_missing_data(ui, ctx.id, "border", "border");
        };

        use too::renderer::Border;
        let border = match border {
            params::Border::Empty => Border::EMPTY,
            params::Border::Thin => Border::THIN,
            params::Border::ThinWide => Border::THIN_WIDE,
            params::Border::Rounded => Border::ROUNDED,
            params::Border::Double => Border::DOUBLE,
            params::Border::Thick => Border::THICK,
            params::Border::ThickTall => Border::THICK_TALL,
            params::Border::ThickWide => Border::THICK_WIDE,
        };

        let view = too::views::border(border).class(params.apply());
        ui.show_children(view, |ui| ctx.visit_children(mapping, ui));
    }
}
