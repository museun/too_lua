use super::Color;
use too::view::{Palette, Style, StyleOptions};

make_proxy! {
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
        ToggleStyle => too::views::ToggleStyle{
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
