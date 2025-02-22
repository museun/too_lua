use anno_lua::Anno;
use mlua::{AnyUserData, FromLua};
use too::view::{Palette, Style, StyleOptions, Ui, ViewExt as _};

use crate::{
    Context, Mapping, MergeStyle, Params, TranslateClass,
    binding::{Spec, View},
    helper::expect_table,
    merge,
};

use super::Color;

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(name = "Checkbox", self)]
pub enum CheckboxClass {
    /// The default style
    #[anno(name = "default")]
    Default,

    /// A markdown inspired style
    #[anno(name = "markdown")]
    Markdown,

    /// An ascii checkbox style
    #[anno(name = "ascii")]
    Ascii,
}

register_enum! {
    CheckboxClass is "Checkbox"
}

impl TranslateClass for CheckboxClass {
    type Style = too::views::CheckboxStyle;

    fn translate(
        &self,
        palette: &Palette,
        options: StyleOptions<<Self::Style as Style>::Args>,
    ) -> Self::Style {
        match self {
            Self::Default => Self::Style::default(palette, options),
            Self::Markdown => Self::Style::markdown(palette, options),
            Self::Ascii => Self::Style::ascii(palette, options),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact, guess)]
pub struct CheckboxStyle {
    /// The character to use when checked
    pub checked: Option<String>,

    /// The character to use when unchecked
    pub unchecked: Option<String>,

    /// The color of the text
    #[anno(lua_type = "Color|string?")]
    pub text_color: Option<Color>,

    /// The color of the text, when hovered
    #[anno(lua_type = "Color|string?")]
    pub hovered_color: Option<Color>,
}

impl FromLua for CheckboxStyle {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        expect_table(&value, |table| {
            Ok(Self {
                checked: table.get("checked")?,
                unchecked: table.get("unchecked")?,
                text_color: table.get("text_color")?,
                hovered_color: table.get("hovered_color")?,
            })
        })
    }
}

impl MergeStyle for CheckboxStyle {
    type Style = too::views::CheckboxStyle;

    fn merge_style(&self, style: &mut Self::Style) {
        merge(&mut style.checked, &self.checked);
        merge(&mut style.unchecked, &self.unchecked);
        merge(&mut style.text_color, &self.text_color);
        merge(&mut style.hovered_color, &self.hovered_color);
    }
}

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact, guess)]
pub struct CheckboxParams {
    /// The style of the checkbox
    #[anno(lua_type = "CheckboxStyle?")]
    pub style: Option<CheckboxStyle>,

    /// The class of the checkbox
    #[anno(lua_type = "Checkbox?")]
    pub class: Option<CheckboxClass>,

    /// The text of the checkbox
    pub text: String,

    /// The state of the checkbox, a boolean
    #[anno(lua_type = "Value")]
    pub value: AnyUserData,
}

impl FromLua for CheckboxParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        expect_table(&value, |table| {
            Ok(Self {
                style: table.get("style")?,
                class: table.get("class")?,
                text: table.get("text")?,
                value: table.get("value")?,
            })
        })
    }
}

impl Params<too::views::CheckboxStyle> for CheckboxParams {
    type Class = CheckboxClass;
    type Style = CheckboxStyle;

    fn class(&self) -> &Option<Self::Class> {
        &self.class
    }
    fn style(&self) -> &Option<Self::Style> {
        &self.style
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Checkbox;

impl View for Checkbox {
    type Params = CheckboxParams;
    type Style = CheckboxStyle;

    fn spec() -> Spec {
        view_spec! {
            /// A checkbox to toggle a boolean
            Self {
                name: "checkbox",
                params: "CheckboxParams"
            }
        }
    }

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params::<CheckboxParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "checkbox", "params");
        };

        let Some(mut value) = ctx.value_mut(&params.value) else {
            return Mapping::report_missing_data(ui, ctx.id, "checkbox", "value");
        };

        let Some(value) = value.bool_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "checkbox", "bool");
        };

        ui.show(
            too::views::checkbox(
                value, //
                &params.text,
            )
            .class(params.apply_styling()),
        );
    }
}
