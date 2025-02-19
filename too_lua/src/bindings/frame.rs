use too::view::{Ui, ViewExt as _};

use crate::{
    bindings::{Align, BorderKind},
    mapping::{BindingSpec, BindingView},
    proxy::Params,
    Context, Mapping,
};

use super::{BorderClass, BorderStyle};

make_struct! {
    struct FrameParams is "FrameParams" {
        /// The style of the border
        style  = Option<BorderStyle> ; "BorderStyle?"
        /// The class of the border
        class  = Option<BorderClass> ; "Border?"
        /// The border to use
        border = BorderKind          ; "BorderKind"
        /// Alignment for the title
        align  = Option<Align>       ; "Align?"
        /// A string to place in the title
        title  = String              ; "string"
    }
}

impl Params<too::views::BorderStyle> for FrameParams {
    type Class = BorderClass;
    type Style = BorderStyle;

    fn class(&self) -> &Option<Self::Class> {
        &self.class
    }
    fn style(&self) -> &Option<Self::Style> {
        &self.style
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Frame;

impl BindingView for Frame {
    const SPEC: BindingSpec = binding! {
        /// A frame, with a title, to surround its children
        "frame" => "FrameParams"
    };

    type Params = FrameParams;
    type Style = BorderStyle;

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        use too::renderer::Border;

        let Some(params) = ctx.params::<FrameParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "frame", "params");
        };

        let align = match params.align.unwrap_or(Align::Middle) {
            Align::Min => too::layout::Align::Min,
            Align::Middle => too::layout::Align::Center,
            Align::Max => too::layout::Align::Max,
        };

        let border = match params.border {
            BorderKind::Empty => Border::EMPTY,
            BorderKind::Thin => Border::THIN,
            BorderKind::ThinWide => Border::THIN_WIDE,
            BorderKind::Rounded => Border::ROUNDED,
            BorderKind::Double => Border::DOUBLE,
            BorderKind::Thick => Border::THICK,
            BorderKind::ThickTall => Border::THICK_TALL,
            BorderKind::ThickWide => Border::THICK_WIDE,
        };

        let view = too::views::frame(border, &params.title).title_align(align);

        ui.show_children(view.class(params.apply_styling()), |ui| {
            ctx.visit_children(mapping, ui)
        });
    }
}
