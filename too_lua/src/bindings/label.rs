use anno_lua::Anno;
use mlua::{Either, FromLua};
use too::view::{Palette, Style, StyleOptions, Ui, ViewExt as _};

use crate::{
    Context, Mapping, TranslateClass,
    binding::{Spec, View},
    bindings::Color,
    helper::get_table,
};

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(name = "Label", self)]
pub enum LabelClass {
    /// The default style
    #[anno(name = "default")]
    Default,

    /// Denotes this label is for information
    #[anno(name = "info")]
    Info,

    /// Denotes this label is for warning
    #[anno(name = "warning")]
    Warning,

    /// Denotes this label is for success
    #[anno(name = "success")]
    Success,

    /// Denotes this label is for danger
    #[anno(name = "danger")]
    Danger,
}

register_enum! {
    LabelClass is "Label"
}

impl TranslateClass for LabelClass {
    type Style = too::views::LabelStyle;

    fn translate(
        &self,
        palette: &Palette,
        options: StyleOptions<<Self::Style as Style>::Args>,
    ) -> Self::Style {
        match self {
            Self::Default => Self::Style::default(palette, options),
            Self::Info => Self::Style::info(palette, options),
            Self::Warning => Self::Style::warning(palette, options),
            Self::Success => Self::Style::success(palette, options),
            Self::Danger => Self::Style::danger(palette, options),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct LabelStyle {
    /// The foreground text color
    #[anno(lua_type = "Color?")]
    pub foreground: Option<Color>,

    /// The text should be italic
    #[anno(lua_type = "boolean?")]
    pub italic: Option<bool>,

    /// The text should be bold
    #[anno(lua_type = "boolean?")]
    pub bold: Option<bool>,

    /// The text should be underline
    #[anno(lua_type = "boolean?")]
    pub underline: Option<bool>,

    /// The text should be faint
    #[anno(lua_type = "boolean?")]
    pub faint: Option<bool>,

    /// The text should be blink
    #[anno(lua_type = "boolean?")]
    pub blink: Option<bool>,

    /// The text should be strikeout
    #[anno(lua_type = "boolean?")]
    pub strikeout: Option<bool>,
}

impl FromLua for LabelStyle {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                foreground: table.get("foreground")?,
                italic: table.get("italic")?,
                bold: table.get("bold")?,
                underline: table.get("underline")?,
                faint: table.get("faint")?,
                blink: table.get("blink")?,
                strikeout: table.get("strikeout")?,
            })
        })
    }
}

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct LabelParams {
    /// The style of the label
    #[anno(lua_type = "LabelStyle?")]
    pub style: Option<LabelStyle>,

    /// The class of the label
    #[anno(lua_type = "Label?")]
    pub class: Option<LabelClass>,

    /// The text of the label
    #[anno(lua_type = "string")]
    pub text: String,
}

impl FromLua for LabelParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                style: table.get("style")?,
                class: table.get("class")?,
                text: table.get("text")?,
            })
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Label;

impl View for Label {
    type Params = LabelParams;
    type Style = LabelStyle;

    fn spec() -> Spec {
        view_spec! {
            /// Label displays some text
            Self {
                name: "label",
                params: "string" | "LabelParams"
            }
        }
    }

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        use too::{
            view::{Palette, StyleOptions},
            views::{Label, LabelStyle, label},
        };

        type Apply = fn(Label) -> Label;
        type Class = fn(&Palette, StyleOptions) -> LabelStyle;

        let Some(params) = ctx.params::<Either<String, LabelParams>>() else {
            return Mapping::report_missing_data(ui, ctx.id, "label", "params");
        };

        let params = match params {
            Either::Left(left) => {
                ui.show(label(left));
                return;
            }
            Either::Right(params) => params,
        };

        let mut label = label(params.text);

        let mut fg = None;
        if let Some(style) = params.style {
            if let Some(Color(new)) = style.foreground {
                fg = Some(new)
            }

            label = [
                style.italic,
                style.bold,
                style.underline,
                style.faint,
                style.blink,
                style.strikeout,
            ]
            .into_iter()
            .map(|c| c.unwrap_or_default())
            .zip([
                Label::italic as Apply,
                Label::bold,
                Label::underline,
                Label::faint,
                Label::blink,
                Label::strikeout,
            ])
            .filter_map(|(v, a)| v.then_some(a))
            .fold(label, |l, a| a(l))
        }

        let mut class: Class = <LabelStyle as Style>::default;
        if let Some(params) = params.class {
            match params {
                LabelClass::Info => class = LabelStyle::info,
                LabelClass::Warning => class = LabelStyle::warning,
                LabelClass::Success => class = LabelStyle::success,
                LabelClass::Danger => class = LabelStyle::danger,
                _ => {}
            };
        }

        let label = if let Some(fg) = fg {
            label.fg(fg)
        } else {
            label.class(class)
        };

        ui.show(label);
    }
}
