use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

use super::{Color, Value};

crate::make_proxy! {
    ToggleParams {
        class:
        ToggleClass is "Toggle" {
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

        style:
        ToggleStyle => too::views::ToggleStyle {
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
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ToggleSwitch;

impl ToggleSwitch {
    binding! {
        /// A switch that is toggled when clicked
        "toggle_switch" => "Value | toggle_switch" {
            /// The style of the selected value
            style "ToggleStyle?"
            /// The class of the selected value
            class "Toggle?"
            /// The state of the selected value, a boolean
            value "Value"
        }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let params = ctx.params::<ToggleParams>();
        let axis = ctx.axis();

        let Some(mut value) = ctx.value_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle", "value");
        };

        let Value::Bool(value) = &mut *value else {
            return Mapping::report_missing(ui, ctx.id, "bool value");
        };

        let mut view = too::views::toggle_switch(value).axis(axis);
        if let Ok(params) = params {
            view = view.class(params.apply())
        }

        ui.show(view);
    }
}
