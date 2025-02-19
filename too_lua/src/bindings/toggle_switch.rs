use mlua::AnyUserData;
use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    mapping::{BindingSpec, BindingView},
    proxy::Params,
    Context, Mapping,
};

use super::{Axis, Color};

make_class! {
    class ToggleSwitchClass is "Toggle" ; too::views::ToggleStyle {
        /// The default style
        Default      = "default"       ; too::views::ToggleStyle::default
        /// A large knob
        Large        = "large"         ; too::views::ToggleStyle::large
        /// A small rounded knob
        SmallRounded = "small_rounded" ; too::views::ToggleStyle::small_rounded
        /// A small diamond knob
        SmallDiamond = "small_diamond" ; too::views::ToggleStyle::small_diamond
        /// A small square knob
        SmallSquare  = "small_square"  ; too::views::ToggleStyle::small_square
    }
}

make_style! {
    style ToggleSwitchStyle is "ToggleSwitchStyle" ; too::views::ToggleStyle {
        /// The character to use for the track
        track             = Option<String> ; "string?"
        /// The color of the track
        track_color       = Option<Color>  ; "Color?"
        /// The color of the track, when hovered
        track_hovered     = Option<Color>  ; "Color?"
        /// The character to use for the knob when its "on"
        on_knob           = Option<String> ; "string?"
        /// The color to use for the knob when its "on"
        on_knob_color     = Option<Color>  ; "Color?"
        /// The character to use for the knob when its "off"
        off_knob          = Option<String> ; "string?"
        /// The color to use for the knob when its "off"
        off_knob_color    = Option<Color>  ; "Color?"
        /// The color to use for the knob when its "on" and hovered
        on_knob_hovered   = Option<Color>  ; "Color?"
        /// The color to use for the knob when its "off" and hovered
        off_knob_hovered  = Option<Color>  ; "Color?"
    }
}

make_struct! {
    struct ToggleSwitchParams is "ToggleSwitchParams" {
        /// The class of the selected value
        class = Option<ToggleSwitchClass> ; "Toggle?"
        /// The style of the selected value
        style = Option<ToggleSwitchStyle> ; "ToggleSwitchStyle?"
        /// The state of the selected value, a boolean
        value = AnyUserData               ; "Value"
        /// Axis for the toggle switch
        axis  = Option<Axis>              ; "Axis?"
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

impl BindingView for ToggleSwitch {
    const SPEC: BindingSpec = binding! {
         /// A switch that is toggled when clicked
         "toggle_switch" => "ToggleSwitchParams"
    };

    type Params = ToggleSwitchParams;
    type Style = ToggleSwitchStyle;

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params::<ToggleSwitchParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle_switch", "params");
        };

        let Some(mut value) = ctx.value_mut(&params.value) else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle_switch", "value");
        };

        let Some(value) = value.bool_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle_switch", "bool");
        };

        let axis = params.axis.unwrap_or_default();
        let view = too::views::toggle_switch(value)
            .axis(axis.into())
            .class(params.apply_styling());

        ui.show(view);
    }
}
