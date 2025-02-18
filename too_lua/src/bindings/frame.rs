use too::view::{Ui, ViewExt as _};

use crate::{
    bindings::{Align, BorderKind, BorderParams},
    mapping::{BindingSpec, BindingView},
    proxy::Params,
    Context, LuaType as _, Mapping,
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
        align  = Align               ; "Align?"
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
        "frame" => FrameParams::NAME
    };

    type Params = FrameParams;
    type Style = BorderStyle;

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<BorderParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "frame", "params");
        };
        let Some(Ok(border)) = ctx.params_field::<BorderKind>("border") else {
            return Mapping::report_missing_data(ui, ctx.id, "frame", "border");
        };
        let Some(Ok(title)) = ctx.params_field::<String>("title") else {
            return Mapping::report_missing_data(ui, ctx.id, "frame", "title");
        };

        let align = ctx
            .params_field::<Align>("align")
            .transpose()
            .ok()
            .flatten()
            .unwrap_or(Align::Middle);

        use too::renderer::Border;
        let border = match border {
            BorderKind::Empty => Border::EMPTY,
            BorderKind::Thin => Border::THIN,
            BorderKind::ThinWide => Border::THIN_WIDE,
            BorderKind::Rounded => Border::ROUNDED,
            BorderKind::Double => Border::DOUBLE,
            BorderKind::Thick => Border::THICK,
            BorderKind::ThickTall => Border::THICK_TALL,
            BorderKind::ThickWide => Border::THICK_WIDE,
        };

        let view = too::views::frame(border, title).title_align(match align {
            Align::Min => too::layout::Align::Min,
            Align::Middle => too::layout::Align::Center,
            Align::Max => too::layout::Align::Max,
        });

        ui.show_children(view.class(params.apply_styling()), |ui| {
            ctx.visit_children(mapping, ui)
        });
    }
}
