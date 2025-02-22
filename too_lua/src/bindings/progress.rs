use anno_lua::Anno;
use mlua::{AnyUserData, Either, FromLua};
use too::view::{Palette, Style, StyleOptions, Ui, ViewExt as _};

use crate::{
    Context, Mapping, MergeStyle, Params, Spec, TranslateClass, View, helper::expect_table, merge,
};

use super::{Axis, Color};

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(name = "Progress", self)]
pub enum ProgressClass {
    /// Default style
    #[anno(name = "default")]
    Default,

    /// A medium filled style
    #[anno(name = "medium_filled")]
    MediumFilled,

    /// A full filled style
    #[anno(name = "filled")]
    Filled,

    /// A thin style
    #[anno(name = "thin")]
    Thin,

    /// A thick style
    #[anno(name = "thick")]
    Thick,

    /// A thin, but dashed style
    #[anno(name = "thin_dashed")]
    ThinDashed,

    /// A thick, but dashed style
    #[anno(name = "thick_dashed")]
    ThickDashed,
}

register_enum! {
    ProgressClass is "Progress"
}

impl TranslateClass for ProgressClass {
    type Style = too::views::ProgressStyle;

    fn translate(
        &self,
        palette: &Palette,
        options: StyleOptions<<Self::Style as Style>::Args>,
    ) -> Self::Style {
        match self {
            Self::Default => Self::Style::default(palette, options),
            Self::MediumFilled => Self::Style::medium_filled(palette, options),
            Self::Filled => Self::Style::filled(palette, options),
            Self::Thin => Self::Style::thin(palette, options),
            Self::Thick => Self::Style::thick(palette, options),
            Self::ThinDashed => Self::Style::thin_dashed(palette, options),
            Self::ThickDashed => Self::Style::thick_dashed(palette, options),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct ProgressStyle {
    /// The unfilled color
    #[anno(lua_type = "Color|string?")]
    pub unfilled_color: Option<Color>,

    /// The filled color
    #[anno(lua_type = "Color|string?")]
    pub filled_color: Option<Color>,

    /// The unfilled color, when hovered
    #[anno(lua_type = "Color|string?")]
    pub unfilled_hovered: Option<Color>,

    /// The filled color, when hovered
    #[anno(lua_type = "Color|string?")]
    pub filled_hovered: Option<Color>,

    /// The character to use for the unfilled portion
    #[anno(lua_type = "string?")]
    pub unfilled: Option<String>,

    /// The character to use for the filled portion
    #[anno(lua_type = "string?")]
    pub filled: Option<String>,
}

impl FromLua for ProgressStyle {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        expect_table(&value, |table| {
            Ok(Self {
                unfilled_color: table.get("unfilled_color")?,
                filled_color: table.get("filled_color")?,
                unfilled_hovered: table.get("unfilled_hovered")?,
                filled_hovered: table.get("filled_hovered")?,
                unfilled: table.get("unfilled")?,
                filled: table.get("filled")?,
            })
        })
    }
}

impl MergeStyle for ProgressStyle {
    type Style = too::views::ProgressStyle;

    fn merge_style(&self, style: &mut Self::Style) {
        merge(&mut style.unfilled_color, &self.unfilled_color);
        merge(&mut style.filled_color, &self.filled_color);
        merge(&mut style.unfilled_hovered, &self.unfilled_hovered);
        merge(&mut style.filled_hovered, &self.filled_hovered);
        merge(&mut style.unfilled, &self.unfilled);
        merge(&mut style.filled, &self.filled);
    }
}

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct ProgressParams {
    /// The style of the progress bar
    #[anno(lua_type = "ProgressStyle?")]
    pub style: Option<ProgressStyle>,

    /// The class of the progress bar
    #[anno(lua_type = "Progress?")]
    pub class: Option<ProgressClass>,

    /// Axis to use for layout
    #[anno(lua_type = "Axis?")]
    pub axis: Option<Axis>,

    /// The value to use (an f32 in the range of 0.0 ..= 1.0)
    #[anno(lua_type = "Value")]
    pub value: AnyUserData,
}

impl FromLua for ProgressParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        expect_table(&value, |table| {
            Ok(Self {
                style: table.get("style")?,
                class: table.get("class")?,
                axis: table.get("axis")?,
                value: table.get("value")?,
            })
        })
    }
}

impl Params<too::views::ProgressStyle> for ProgressParams {
    type Class = ProgressClass;
    type Style = ProgressStyle;

    fn class(&self) -> &Option<Self::Class> {
        &self.class
    }
    fn style(&self) -> &Option<Self::Style> {
        &self.style
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Progress;

impl View for Progress {
    type Params = ProgressParams;
    type Style = ProgressStyle;

    fn spec() -> Spec {
        view_spec! {
            /// A progress bar
            Self {
                name: "progress",
                params: "Value" | "ProgressParams"
            }
        }
    }

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params::<Either<AnyUserData, ProgressParams>>() else {
            return Mapping::report_missing_data(ui, ctx.id, "progress", "params");
        };

        let value = match &params {
            Either::Left(value) => value,
            Either::Right(params) => &params.value,
        };

        let Some(value) = ctx.value_ref(value) else {
            return Mapping::report_missing_data(ui, ctx.id, "progress", "value");
        };

        let Some(value) = value.float_ref() else {
            return Mapping::report_missing_data(ui, ctx.id, "progress", "float");
        };

        let mut view = too::views::progress(*value);
        if let Either::Right(params) = params {
            view = view
                .axis(params.axis.unwrap_or_default().into())
                .class(params.apply_styling());
        }
        ui.show(view);
    }
}
