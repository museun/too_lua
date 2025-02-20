use anno_lua::Anno;
use mlua::FromLua;
use too::view::{Palette, Style, StyleOptions, Ui, ViewExt as _};

use crate::{
    Context, Mapping, MergeStyle, Params, TranslateClass,
    binding::{Spec, View},
    bindings::Color,
    helper::get_table,
    merge,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Anno)]
#[anno(self)]
pub enum BorderKind {
    /// No border
    #[anno(name = "empty")]
    Empty,

    /// A thin border
    #[anno(name = "thin")]
    Thin,

    /// A thin, but wide border
    #[anno(name = "thin_wide")]
    ThinWide,

    /// A rounded border
    #[anno(name = "rounded")]
    Rounded,

    /// A double-line border
    #[anno(name = "double")]
    Double,

    /// A thick border
    #[anno(name = "thick")]
    #[default]
    Thick,

    /// A thick, but tall border
    #[anno(name = "thick_tall")]
    ThickTall,

    /// A thick, but wide border
    #[anno(name = "thick_wide")]
    ThickWide,
}

impl From<BorderKind> for too::renderer::Border {
    fn from(value: BorderKind) -> Self {
        match value {
            BorderKind::Empty => Self::EMPTY,
            BorderKind::Thin => Self::THIN,
            BorderKind::ThinWide => Self::THIN_WIDE,
            BorderKind::Rounded => Self::ROUNDED,
            BorderKind::Double => Self::DOUBLE,
            BorderKind::Thick => Self::THICK,
            BorderKind::ThickTall => Self::THICK_TALL,
            BorderKind::ThickWide => Self::THICK_WIDE,
        }
    }
}

register_enum! {
    BorderKind is "BorderKind"
}

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(name = "Border", self)]
pub enum BorderClass {
    /// The default style
    #[anno(name = "default")]
    Default,

    /// An interactive style
    #[anno(name = "interactive")]
    Interactive,
}

register_enum! {
    BorderClass is "Border"
}

impl TranslateClass for BorderClass {
    type Style = too::views::BorderStyle;

    fn translate(
        &self,
        palette: &Palette,
        options: StyleOptions<<Self::Style as Style>::Args>,
    ) -> Self::Style {
        match self {
            Self::Default => Self::Style::default(palette, options),
            Self::Interactive => Self::Style::interactive(palette, options),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct BorderStyle {
    /// The frame title color
    #[anno(lua_type = "Color?")]
    pub title: Option<Color>,

    /// The color of the border
    #[anno(lua_type = "Color?")]
    pub border: Option<Color>,

    /// The color of the border, when focused
    #[anno(lua_type = "Color?")]
    pub border_focused: Option<Color>,

    /// The color of the border, when hovered
    #[anno(lua_type = "Color?")]
    pub border_hovered: Option<Color>,
}

impl MergeStyle for BorderStyle {
    type Style = too::views::BorderStyle;

    fn merge_style(&self, style: &mut Self::Style) {
        merge(&mut style.title, &self.title);
        merge(&mut style.border, &self.border);
        merge(&mut style.border_focused, &self.border_focused);
        merge(&mut style.border_hovered, &self.border_hovered);
    }
}

impl FromLua for BorderStyle {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                title: table.get("title")?,
                border: table.get("border")?,
                border_focused: table.get("border_focused")?,
                border_hovered: table.get("border_hovered")?,
            })
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct BorderParams {
    /// The style of the border
    #[anno(lua_type = "BorderStyle?")]
    pub style: Option<BorderStyle>,

    /// The class of the border
    #[anno(lua_type = "Border?")]
    pub class: Option<BorderClass>,

    /// The border to use
    #[anno(lua_type = "BorderKind")]
    pub border: BorderKind,
}

impl FromLua for BorderParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                style: table.get("style")?,
                class: table.get("class")?,
                border: table.get("border")?,
            })
        })
    }
}

impl Params<too::views::BorderStyle> for BorderParams {
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
pub struct Border;

impl View for Border {
    type Params = BorderParams;
    type Style = BorderStyle;

    fn spec() -> Spec {
        view_spec! {
            /// Border to surround its children
            Self {
                name: "border",
                params: "BorderParams"
            }
        }
    }

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params::<BorderParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "border", "params");
        };

        let view = too::views::border(params.border.into()) //
            .class(params.apply_styling());
        ui.show_children(view, |ui| ctx.visit_children(mapping, ui));
    }
}
