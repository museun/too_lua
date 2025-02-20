use anno_lua::Anno;
use mlua::{FromLua, LuaSerdeExt};
use too::view::{Palette, Style, StyleOptions, Ui, ViewExt as _};

use crate::{binding::View, merge, Context, Mapping, MergeStyle, Params, TranslateClass};

use super::Color;

#[derive(Copy, Clone, Debug, PartialEq, Anno, serde::Deserialize)]
#[anno(name = "Button", self)]
pub enum ButtonClass {
    #[anno(name = "default")]
    #[serde(rename = "default")]
    Default,

    /// Denotes this button is for success
    #[anno(name = "success")]
    #[serde(rename = "success")]
    Success,

    /// Denotes this button is for information
    #[anno(name = "info")]
    #[serde(rename = "info")]
    Info,

    /// Denotes this button is for warning
    #[anno(name = "warning")]
    #[serde(rename = "warning")]
    Warning,

    /// Denotes this button is for danger
    #[anno(name = "danger")]
    #[serde(rename = "danger")]
    Danger,
}

register_enum! {
    ButtonClass is "Button"
}

impl TranslateClass for ButtonClass {
    type Style = too::views::ButtonStyle;

    fn translate(
        &self,
        palette: &Palette,
        options: StyleOptions<<Self::Style as Style>::Args>,
    ) -> Self::Style {
        match self {
            Self::Default => Self::Style::default(palette, options),
            Self::Success => Self::Style::success(palette, options),
            Self::Info => Self::Style::info(palette, options),
            Self::Warning => Self::Style::warning(palette, options),
            Self::Danger => Self::Style::danger(palette, options),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Anno, serde::Deserialize)]
#[anno(exact)]
pub struct ButtonStyle {
    /// The button text color
    #[anno(lua_type = "Color?")]
    pub text_color: Option<Color>,

    /// The button background color
    #[anno(lua_type = "Color?")]
    pub background: Option<Color>,
}

impl MergeStyle for ButtonStyle {
    type Style = too::views::ButtonStyle;

    fn merge_style(&self, style: &mut Self::Style) {
        merge(&mut style.text_color, &self.text_color);
        merge(&mut style.background, &self.background);
    }
}

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct ButtonParams {
    /// The style of the button
    #[anno(lua_type = "ButtonStyle?")]
    pub style: Option<ButtonStyle>,

    /// The class of the button
    #[anno(lua_type = "Button?")]
    pub class: Option<ButtonClass>,

    /// The text of the button
    #[anno(lua_type = "string")]
    pub text: String,

    /// Function to call when the button is clicked
    #[anno(lua_type = "fun(): nil")]
    pub handler: mlua::Function,
}

impl FromLua for ButtonParams {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(table) = value else {
            return Err(mlua::Error::runtime(format!(
                "expected ButtonParams, got: {}",
                value.type_name()
            )));
        };

        Ok(Self {
            style: lua.from_value(table.get("style")?)?,
            class: lua.from_value(table.get("class")?)?,
            text: table.get("text")?,
            handler: table.get("handler")?,
        })
    }
}

impl Params<too::views::ButtonStyle> for ButtonParams {
    type Class = ButtonClass;
    type Style = ButtonStyle;

    fn class(&self) -> &Option<Self::Class> {
        &self.class
    }
    fn style(&self) -> &Option<Self::Style> {
        &self.style
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Button;

impl View for Button {
    type Params = ButtonParams;
    type Style = ButtonStyle;

    fn spec() -> crate::binding::Spec {
        view_spec! {
            /// A button to click
            Self {
                name: "button",
                params: "ButtonParams"
            }
        }
    }

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params::<ButtonParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "button", "params");
        };

        let handler = params.handler.clone();
        let view = too::views::button(&params.text).class(params.apply_styling());
        if ui.show(view).clicked() {
            let _ = handler.call::<()>(());
        }
    }
}
