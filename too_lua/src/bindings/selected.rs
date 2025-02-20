use anno_lua::Anno;
use mlua::{AnyUserData, FromLua};
use too::view::{Palette, Style, StyleOptions, Ui, ViewExt as _};

use crate::{
    Context, Mapping, MergeStyle, Params, Spec, TranslateClass, View, helper::get_table, merge,
};

use super::Color;

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(name = "Selected", self)]
pub enum SelectedClass {
    /// The default style
    #[anno(name = "default")]
    Default,

    /// This element reacts to hovers
    #[anno(name = "hovered")]
    Hovered,
}

register_enum! {
    SelectedClass is "Selected"
}

impl TranslateClass for SelectedClass {
    type Style = too::views::SelectedStyle;

    fn translate(
        &self,
        palette: &Palette,
        options: StyleOptions<<Self::Style as Style>::Args>,
    ) -> Self::Style {
        match self {
            Self::Default => Self::Style::default(palette, options),
            Self::Hovered => Self::Style::hovered(palette, options),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct SelectedStyle {
    /// The background color
    #[anno(lua_type = "Color?")]
    pub background: Option<Color>,

    /// The text color
    #[anno(lua_type = "Color?")]
    pub text_color: Option<Color>,

    /// The background color, when selected
    #[anno(lua_type = "Color?")]
    pub selected_background: Option<Color>,

    /// The text color, when hovered
    #[anno(lua_type = "Color?")]
    pub hovered_text: Option<Color>,

    /// The background color, when hovered
    #[anno(lua_type = "Color?")]
    pub hovered_background: Option<Color>,
}

impl FromLua for SelectedStyle {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                background: table.get("background")?,
                text_color: table.get("text_color")?,
                selected_background: table.get("selected_background")?,
                hovered_text: table.get("hovered_text")?,
                hovered_background: table.get("hovered_background")?,
            })
        })
    }
}

impl MergeStyle for SelectedStyle {
    type Style = too::views::SelectedStyle;

    fn merge_style(&self, style: &mut Self::Style) {
        merge(&mut style.background, &self.background);
        merge(&mut style.text_color, &self.text_color);
        merge(&mut style.selected_background, &self.selected_background);
        merge(&mut style.hovered_text, &self.hovered_text);
        merge(&mut style.hovered_background, &self.hovered_background);
    }
}

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct SelectedParams {
    /// The style of the selected value
    #[anno(lua_type = "SelectedStyle?")]
    pub style: Option<SelectedStyle>,

    /// The class of the selected value
    #[anno(lua_type = "Selected?")]
    pub class: Option<SelectedClass>,

    /// The text of the selected value
    #[anno(lua_type = "string")]
    pub text: String,

    /// The state of the selected value, a boolean
    #[anno(lua_type = "Value")]
    pub value: AnyUserData,
}

impl FromLua for SelectedParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                style: table.get("style")?,
                class: table.get("class")?,
                text: table.get("text")?,
                value: table.get("value")?,
            })
        })
    }
}

impl Params<too::views::SelectedStyle> for SelectedParams {
    type Class = SelectedClass;
    type Style = SelectedStyle;

    fn class(&self) -> &Option<Self::Class> {
        &self.class
    }
    fn style(&self) -> &Option<Self::Style> {
        &self.style
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Selected;

impl View for Selected {
    type Params = SelectedParams;
    type Style = SelectedStyle;

    fn spec() -> Spec {
        view_spec! {
            /// A selected boolean value
            Self {
                name: "selected",
                params: "SelectedParams"
            }
        }
    }

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params::<SelectedParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "selected", "params");
        };

        let Some(mut value) = ctx.value_mut(&params.value) else {
            return Mapping::report_missing_data(ui, ctx.id, "selected", "value");
        };

        let Some(value) = value.bool_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "selected", "bool");
        };

        ui.show(too::views::selected(value, &params.text).class(params.apply_styling()));
    }
}
