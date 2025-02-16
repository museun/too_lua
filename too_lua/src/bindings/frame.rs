use too::view::{Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    params, Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Frame;

impl Frame {
    binding! {
        /// Frame is a border with a title
        "frame" => "frame" {
           /// The style of the frame
           style "BorderStyle?"
           /// The class of the frame
           class "Border?"
           /// The border to use
           border "BorderKind"
           /// Alignment for the title
           align "Align?"
           /// A string to place in the title
           title "string"
        }
    }

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        {
            let Ok(params) = ctx.params::<params::BorderParams>() else {
                return Mapping::report_missing_data(ui, ctx.id, "frame", "params");
            };
            let Some(Ok(border)) = ctx.params_field::<params::Border>("border") else {
                return Mapping::report_missing_data(ui, ctx.id, "frame", "border");
            };
            let Some(Ok(title)) = ctx.params_field::<String>("title") else {
                return Mapping::report_missing_data(ui, ctx.id, "frame", "title");
            };

            let align = ctx
                .params_field::<params::Align>("align")
                .transpose()
                .ok()
                .flatten()
                .unwrap_or(params::Align::Middle);

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

            let view = too::views::frame(border, title).title_align(match align {
                params::Align::Min => too::layout::Align::Min,
                params::Align::Middle => too::layout::Align::Center,
                params::Align::Max => too::layout::Align::Max,
            });

            ui.show_children(view.class(params.apply()), |ui| {
                ctx.visit_children(mapping, ui)
            });
        }
    }
}
