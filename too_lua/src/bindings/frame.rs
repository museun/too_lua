use anno_lua::Anno;
use mlua::FromLua;
use too::view::{Ui, ViewExt as _};

use crate::{
    Context, Mapping, Params, Spec, View,
    bindings::{Align, BorderKind},
    helper::expect_table,
};

use super::{BorderClass, BorderStyle};

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact, guess)]
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
    pub title: String,
}

impl FromLua for FrameParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        expect_table(&value, |table| {
            Ok(Self {
                style: table.get("style")?,
                class: table.get("class")?,
                border: table.get("border")?,
                align: table.get("align")?,
                title: table.get("title")?,
            })
        })
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
        let Some(params) = ctx.params::<FrameParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "frame", "params");
        };

        let align = params.align.unwrap_or(Align::Center);
        let view = too::views::frame(params.border.into(), &params.title) //
            .title_align(align.into());

        ui.show_children(view.class(params.apply_styling()), |ui| {
            ctx.visit_children(mapping, ui)
        });
    }
}
