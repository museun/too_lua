use anno_lua::Anno;
use mlua::{AnyUserData, FromLua};
use too::view::{Palette, Style, StyleOptions, Ui, ViewExt as _};

use crate::{
    helper::get_table, merge, Context, Mapping, MergeStyle, Params, Spec, TranslateClass, View,
};

use super::{Axis, Color};

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(name = "Slider", self)]
pub enum SliderClass {
    /// The default style
    #[anno(name = "default")]
    Default,

    /// Small track and rounded knob
    #[anno(name = "small_rounded")]
    SmallRounded,

    /// Small track and diamond knob
    #[anno(name = "small_diamond")]
    SmallDiamond,

    /// Small track and square knob
    #[anno(name = "small_square")]
    SmallSquare,

    /// Medium track and large knob
    #[anno(name = "large")]
    Large,

    /// Large track and large knob
    #[anno(name = "large_filled")]
    LargeFilled,
}

register_enum! {
    SliderClass is "Slider"
}

impl TranslateClass for SliderClass {
    type Style = too::views::SliderStyle;

    fn translate(
        &self,
        palette: &Palette,
        options: StyleOptions<<Self::Style as Style>::Args>,
    ) -> Self::Style {
        match self {
            Self::Default => Self::Style::default(palette, options),
            Self::SmallRounded => Self::Style::small_rounded(palette, options),
            Self::SmallDiamond => Self::Style::small_diamond(palette, options),
            Self::SmallSquare => Self::Style::small_square(palette, options),
            Self::Large => Self::Style::large(palette, options),
            Self::LargeFilled => Self::Style::large_filled(palette, options),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct SliderStyle {
    /// The color of the track
    #[anno(lua_type = "Color?")]
    pub track_color: Option<Color>,

    /// The color of the knob
    #[anno(lua_type = "Color?")]
    pub knob_color: Option<Color>,

    /// The color of the track, when hovered
    #[anno(lua_type = "Color?")]
    pub track_hovered: Option<Color>,

    /// The color of the knob, when hovered
    #[anno(lua_type = "Color?")]
    pub knob_hovered: Option<Color>,

    /// The character to use for the knob
    #[anno(lua_type = "string?")]
    pub knob: Option<String>,

    /// The character to use for the track
    #[anno(lua_type = "string?")]
    pub track: Option<String>,
}

impl FromLua for SliderStyle {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                track_color: table.get("track_color")?,
                knob_color: table.get("knob_color")?,
                track_hovered: table.get("track_hovered")?,
                knob_hovered: table.get("knob_hovered")?,
                knob: table.get("knob")?,
                track: table.get("track")?,
            })
        })
    }
}

impl MergeStyle for SliderStyle {
    type Style = too::views::SliderStyle;

    fn merge_style(&self, style: &mut Self::Style) {
        merge(&mut style.track_color, &self.track_color);
        merge(&mut style.knob_color, &self.knob_color);
        merge(&mut style.track_hovered, &self.track_hovered);
        merge(&mut style.knob_hovered, &self.knob_hovered);
        merge(&mut style.knob, &self.knob);
        merge(&mut style.track, &self.track);
    }
}

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct SliderParams {
    /// The style of the slider
    #[anno(lua_type = "SliderStyle?")]
    pub style: Option<SliderStyle>,

    /// The class of the slider
    #[anno(lua_type = "Slider?")]
    pub class: Option<SliderClass>,

    /// Axis to use for layout
    #[anno(lua_type = "Axis?")]
    pub axis: Option<Axis>,

    /// The value to use (an f32 in the range of 0.0 ..= 1.0)
    #[anno(lua_type = "Value")]
    pub value: AnyUserData,
}

impl FromLua for SliderParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                style: table.get("style")?,
                class: table.get("class")?,
                axis: table.get("axis")?,
                value: table.get("value")?,
            })
        })
    }
}

impl Params<too::views::SliderStyle> for SliderParams {
    type Class = SliderClass;
    type Style = SliderStyle;

    fn class(&self) -> &Option<Self::Class> {
        &self.class
    }
    fn style(&self) -> &Option<Self::Style> {
        &self.style
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Slider;

impl View for Slider {
    type Params = SliderParams;
    type Style = SliderStyle;

    fn spec() -> Spec {
        view_spec! {
            /// A slider to adjust a value
            Self {
                name: "slider",
                params: "SliderParams"
            }
        }
    }

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params::<SliderParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "slider", "params");
        };

        let Some(mut value) = ctx.value_mut(&params.value) else {
            return Mapping::report_missing_data(ui, ctx.id, "slider", "value");
        };

        let Some(value) = value.float_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "slider", "float");
        };

        let axis = params.axis.unwrap_or_default();
        let view = too::views::slider(value)
            .axis(axis.into())
            .class(params.apply_styling());
        ui.show(view);
    }
}
