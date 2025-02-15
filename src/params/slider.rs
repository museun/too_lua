use super::Color;
use too::view::{Palette, Style, StyleOptions};

make_proxy! {
    SliderParams {
        class:
        SliderClass {
            Default      = "default"       ; too::views::SliderStyle::default
            SmallRounded = "small_rounded" ; too::views::SliderStyle::small_rounded
            SmallDiamond = "small_diamond" ; too::views::SliderStyle::small_diamond
            SmallSquare  = "small_square"  ; too::views::SliderStyle::small_square
            Large        = "large"         ; too::views::SliderStyle::large
            LargeFilled  = "large_filled"  ; too::views::SliderStyle::large_filled
        }

        style:
        SliderStyle => too::views::SliderStyle {
            track_color   = Option<Color>
            knob_color    = Option<Color>
            track_hovered = Option<Color>
            knob_hovered  = Option<Color>
            knob          = Option<String>
            track         = Option<String>
        }
    }
}
