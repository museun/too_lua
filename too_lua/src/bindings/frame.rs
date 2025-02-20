use anno_lua::Anno;
use too::view::{Ui, ViewExt as _};

use crate::{
    bindings::{Align, BorderKind},
    Context, Mapping, Params, Spec, View,
};

use super::{BorderClass, BorderStyle};

#[derive(Clone, Debug, PartialEq, Anno, serde::Deserialize)]
#[anno(exact)]
pub struct FrameParams {
    /// The style of the border
    #[anno(lua_type = "BorderStyle?")]
    pub style: Option<BorderStyle>,

    /// The class of the border
    #[anno(lua_type = "Border?")]
    pub class: Option<BorderClass>,

    /// The border to use
    #[anno(lua_type = "BorderKind")]
    pub border: BorderKind,

    /// Alignment for the title
    #[anno(lua_type = "Align?")]
    pub align: Option<Align>,

    /// A string to place in the title
    #[anno(lua_type = "string")]
    pub title: String,
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

impl View for Frame {
    type Params = FrameParams;
    type Style = BorderStyle;

    fn spec() -> Spec {
        view_spec! {
            /// A frame, with a title, to surround its children
            Self {
                name: "frame",
                params: "FrameParams"
            }
        }
    }

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params_de::<FrameParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "frame", "params");
        };

        let align = params.align.unwrap_or(Align::Middle);
        let view = too::views::frame(params.border.into(), &params.title) //
            .title_align(align.into());

        ui.show_children(view.class(params.apply_styling()), |ui| {
            ctx.visit_children(mapping, ui)
        });
    }
}
