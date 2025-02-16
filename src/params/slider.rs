use super::Color;
use too::view::{Palette, Style, StyleOptions};

make_proxy! {
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
