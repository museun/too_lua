use anno_lua::Anno;
use mlua::{AnyUserData, FromLua};
use too::view::{Palette, Style, StyleOptions, Ui, ViewExt as _};

use crate::{Context, Mapping, Spec, TranslateClass, View, helper::expect_table};

use super::Color;

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(name = "Todo", self)]
pub enum TodoClass {
    /// The default style
    #[anno(name = "default")]
    Default,
}

register_enum! {
    TodoClass is "Todo"
}

impl TranslateClass for TodoClass {
    type Style = too::views::TodoStyle;

    fn translate(
        &self,
        palette: &Palette,
        options: StyleOptions<<Self::Style as Style>::Args>,
    ) -> Self::Style {
        match self {
            Self::Default => Self::Style::default(palette, options),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact, guess)]
pub struct TodoStyle {
    /// When selected, the text should be bold
    pub bold: Option<bool>,

    /// When selected, the text should be faint
    pub faint: Option<bool>,

    /// When selected, the text should be italic
    pub italic: Option<bool>,

    /// When selected, the text should be underline
    pub underline: Option<bool>,

    /// When selected, the text should be blink
    pub blink: Option<bool>,

    /// When selected, the text should be reverse
    pub reverse: Option<bool>,

    /// When selected, the text should be strikeout
    pub strikeout: Option<bool>,

    /// The color of the text
    #[anno(lua_type = "Color|string?")]
    pub text_color: Option<Color>,

    /// The color of the text, when hovered
    #[anno(lua_type = "Color|string?")]
    pub hovered_color: Option<Color>,
}

impl FromLua for TodoStyle {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        expect_table(&value, |table| {
            Ok(Self {
                bold: table.get("bold")?,
                faint: table.get("faint")?,
                italic: table.get("italic")?,
                underline: table.get("underline")?,
                blink: table.get("blink")?,
                reverse: table.get("reverse")?,
                strikeout: table.get("strikeout")?,
                text_color: table.get("text_color")?,
                hovered_color: table.get("hovered_color")?,
            })
        })
    }
}

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact, guess)]
pub struct TodoParams {
    /// The class of the selected value
    #[anno(lua_type = "Todo?")]
    pub class: Option<TodoClass>,

    /// The style of the selected value
    #[anno(lua_type = "TodoStyle?")]
    pub style: Option<TodoStyle>,

    /// The text of the selected value
    pub text: String,

    /// The state of the selected value, a boolean
    #[anno(lua_type = "Value")]
    pub value: AnyUserData,
}

impl FromLua for TodoParams {
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TodoValue;

impl View for TodoValue {
    type Params = TodoParams;
    type Style = TodoStyle;

    fn spec() -> Spec {
        view_spec! {
            /// A selected value
            Self {
                name: "todo_value",
                params: "TodoParams"
            }
        }
    }

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params::<TodoParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "todo", "params");
        };

        let Some(mut value) = ctx.value_mut(&params.value) else {
            return Mapping::report_missing_data(ui, ctx.id, "todo", "value");
        };

        let Some(value) = value.bool_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "todo", "bool");
        };

        let default = <too::views::TodoStyle as Style>::default;

        let view = too::views::todo_value(value, &params.text);
        let class = params
            .class
            .and_then(|class| {
                #[allow(unreachable_patterns)]
                let val = match class {
                    TodoClass::Default => default,
                    _ => return None,
                };
                Some(val)
            })
            .unwrap_or(default);

        let mut attr = None;
        let mut text_color = None;
        let mut hovered_color = None;

        use too::renderer::Attribute;
        if let Some(style) = params.style {
            let new = [
                (style.italic, Attribute::ITALIC),
                (style.bold, Attribute::BOLD),
                (style.underline, Attribute::UNDERLINE),
                (style.faint, Attribute::FAINT),
                (style.blink, Attribute::BLINK),
                (style.strikeout, Attribute::STRIKEOUT),
            ]
            .into_iter()
            .filter_map(|(c, a)| c.unwrap_or_default().then_some(a))
            .fold(Attribute::RESET, |l, a| l | a);

            attr = Some(new).filter(|c| !c.is_reset());
            text_color = style.text_color.map(|c| c.0);
            hovered_color = style.hovered_color.map(|c| c.0);
        }

        ui.show(view.class(move |palette, options| {
            let mut style = class(palette, options);
            if let Some(attr) = attr {
                style.selected = attr;
            }
            if let Some(text_color) = text_color {
                style.text_color = text_color;
            }
            if let Some(hovered_color) = hovered_color {
                style.hovered_color = Some(hovered_color);
            }
            style
        }));
    }
}
