use mlua::AnyUserData;
use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    mapping::{BindingSpec, BindingView},
    proxy::Params,
    Context, LuaType as _, Mapping,
};

use super::{Color, Value};

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
         "toggle_switch" => ToggleSwitchParams::NAME
    };

    type Params = ToggleSwitchParams;
    type Style = ToggleSwitchStyle;

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let params = ctx.params::<ToggleSwitchParams>();
        let axis = ctx.axis();

        let Some(mut value) = ctx.value_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle", "value");
        };

        let Value::Bool(value) = &mut *value else {
            return Mapping::report_missing(ui, ctx.id, "bool value");
        };

        let mut view = too::views::toggle_switch(value).axis(axis);
        if let Ok(params) = params {
            view = view.class(params.apply_styling())
        }

        ui.show(view);
    }
}
