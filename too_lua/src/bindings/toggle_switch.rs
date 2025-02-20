use anno_lua::Anno;
use mlua::{AnyUserData, Either, FromLua};
use too::view::{Palette, Style, StyleOptions, Ui, ViewExt as _};

use crate::{
    helper::get_table, merge, Context, Mapping, MergeStyle, Params, Spec, TranslateClass, View,
};

use super::{Axis, Color};

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(name = "Toggle", self)]
pub enum ToggleSwitchClass {
    /// The default style
    #[anno(name = "default")]
    Default,

    /// A large knob
    #[anno(name = "large")]
    Large,

    /// A small rounded knob
    #[anno(name = "small_rounded")]
    SmallRounded,

    /// A small diamond knob
    #[anno(name = "small_diamond")]
    SmallDiamond,

    /// A small square knob
    #[anno(name = "small_square")]
    SmallSquare,
}

register_enum! {
    ToggleSwitchClass is "Toggle"
}

impl TranslateClass for ToggleSwitchClass {
    type Style = too::views::ToggleStyle;

    fn translate(
        &self,
        palette: &Palette,
        options: StyleOptions<<Self::Style as Style>::Args>,
    ) -> Self::Style {
        match self {
            Self::Default => Self::Style::default(palette, options),
            Self::Large => Self::Style::large(palette, options),
            Self::SmallRounded => Self::Style::small_rounded(palette, options),
            Self::SmallDiamond => Self::Style::small_diamond(palette, options),
            Self::SmallSquare => Self::Style::small_square(palette, options),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct ToggleSwitchStyle {
    /// The character to use for the track
    #[anno(lua_type = "string?")]
    pub track: Option<String>,

    /// The color of the track
    #[anno(lua_type = "Color?")]
    pub track_color: Option<Color>,

    /// The color of the track, when hovered
    #[anno(lua_type = "Color?")]
    pub track_hovered: Option<Color>,

    /// The character to use for the knob when its "on"
    #[anno(lua_type = "string?")]
    pub on_knob: Option<String>,

    /// The color to use for the knob when its "on"
    #[anno(lua_type = "Color?")]
    pub on_knob_color: Option<Color>,

    /// The character to use for the knob when its "off"
    #[anno(lua_type = "string?")]
    pub off_knob: Option<String>,

    /// The color to use for the knob when its "off"
    #[anno(lua_type = "Color?")]
    pub off_knob_color: Option<Color>,

    /// The color to use for the knob when its "on" and hovered
    #[anno(lua_type = "Color?")]
    pub on_knob_hovered: Option<Color>,

    /// The color to use for the knob when its "off" and hovered
    #[anno(lua_type = "Color?")]
    pub off_knob_hovered: Option<Color>,
}

impl FromLua for ToggleSwitchStyle {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                track: table.get("track")?,
                track_color: table.get("track_color")?,
                track_hovered: table.get("track_hovered")?,
                on_knob: table.get("on_knob")?,
                on_knob_color: table.get("on_knob_color")?,
                off_knob: table.get("off_knob")?,
                off_knob_color: table.get("off_knob_color")?,
                on_knob_hovered: table.get("on_knob_hovered")?,
                off_knob_hovered: table.get("off_knob_hovered")?,
            })
        })
    }
}

impl MergeStyle for ToggleSwitchStyle {
    type Style = too::views::ToggleStyle;

    fn merge_style(&self, style: &mut Self::Style) {
        merge(&mut style.track, &self.track);
        merge(&mut style.track_color, &self.track_color);
        merge(&mut style.track_hovered, &self.track_hovered);
        merge(&mut style.on_knob, &self.on_knob);
        merge(&mut style.on_knob_color, &self.on_knob_color);
        merge(&mut style.off_knob, &self.off_knob);
        merge(&mut style.off_knob_color, &self.off_knob_color);
        merge(&mut style.on_knob_hovered, &self.on_knob_hovered);
        merge(&mut style.off_knob_hovered, &self.off_knob_hovered);
    }
}

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct ToggleSwitchParams {
    /// The class of the selected value
    #[anno(lua_type = "Toggle?")]
    pub class: Option<ToggleSwitchClass>,

    /// The style of the selected value
    #[anno(lua_type = "ToggleSwitchStyle?")]
    pub style: Option<ToggleSwitchStyle>,

    /// The state of the selected value, a boolean
    #[anno(lua_type = "Value")]
    pub value: AnyUserData,

    /// Axis for the toggle switch
    #[anno(lua_type = "Axis?")]
    pub axis: Option<Axis>,
}

impl FromLua for ToggleSwitchParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                style: table.get("style")?,
                class: table.get("class")?,
                value: table.get("value")?,
                axis: table.get("axis")?,
            })
        })
    }
}

impl Params<too::views::ToggleStyle> for ToggleSwitchParams {
    type Class = ToggleSwitchClass;
    type Style = ToggleSwitchStyle;

    fn class(&self) -> &Option<Self::Class> {
        &self.class
    }
    fn style(&self) -> &Option<Self::Style> {
        &self.style
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ToggleSwitch;

impl View for ToggleSwitch {
    type Params = ToggleSwitchParams;
    type Style = ToggleSwitchStyle;

    fn spec() -> Spec {
        view_spec! {
            /// A switch that is toggled when clicked
            Self {
                name: "toggle_switch",
                params: "Value" | "ToggleSwitchParams"
            }
        }
    }

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params::<Either<AnyUserData, ToggleSwitchParams>>() else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle_switch", "params");
        };

        let value = match &params {
            Either::Left(value) => value,
            Either::Right(params) => &params.value,
        };

        let Some(mut value) = ctx.value_mut(&value) else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle_switch", "value");
        };

        let Some(value) = value.bool_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle_switch", "bool");
        };

        let mut view = too::views::toggle_switch(value);
        if let Either::Right(params) = params {
            view = view
                .axis(params.axis.unwrap_or_default().into())
                .class(params.apply_styling());
        }
        ui.show(view);
    }
}
