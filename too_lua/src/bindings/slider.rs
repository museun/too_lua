use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

use super::{Color, Value};

crate::make_proxy! {
    SliderParams {
        class:
        SliderClass is "Slider" {
            /// The default style
            Default      = "default"       ; too::views::SliderStyle::default
            /// Small track and rounded knob
            SmallRounded = "small_rounded" ; too::views::SliderStyle::small_rounded
            /// Small track and diamond knob
            SmallDiamond = "small_diamond" ; too::views::SliderStyle::small_diamond
            /// Small track and square knob
            SmallSquare  = "small_square"  ; too::views::SliderStyle::small_square
            /// Medium track and large knob
            Large        = "large"         ; too::views::SliderStyle::large
            /// Large track and large knob
            LargeFilled  = "large_filled"  ; too::views::SliderStyle::large_filled
        }

        style:
        SliderStyle => too::views::SliderStyle {
            /// The color of the track
            track_color   = Option<Color>  ; "Color?"
            /// The color of the knob
            knob_color    = Option<Color>  ; "Color?"
            /// The color of the track, when hovered
            track_hovered = Option<Color>  ; "Color?"
            /// The color of the knob, when hovered
            knob_hovered  = Option<Color>  ; "Color?"
            /// The character to use for the knob
            knob          = Option<String> ; "string?"
            /// The character to use for the track
            track         = Option<String> ; "string?"
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Slider;

impl Slider {
    binding! {
        /// A slider to adjust a value
        "slider" => "Value | slider" {
            /// The style of the slider
            style "SliderStyle?"
            /// The class of the slider
            class "Slider?"
            /// Axis to use for layout
            axis "Axis?"
            /// The value to use (an f32 in the range of 0.0 ..= 1.0)
            value "Value"
        }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let params = ctx.params::<SliderParams>();
        let axis = ctx.axis();

        let Some(mut v) = ctx.value_mut() else {
            return Mapping::report_missing(ui, ctx.id, "slider");
        };
        let Value::Float(v) = &mut *v else {
            return Mapping::report_missing(ui, ctx.id, "float value");
        };

        let mut view = too::views::slider(v).axis(axis);
        if let Ok(params) = params {
            view = view.class(params.apply())
        }
        ui.show(view);
    }
}
