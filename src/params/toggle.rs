use super::Color;
use too::view::{Palette, Style, StyleOptions};

make_proxy! {
    ToggleParams {
        class:
        ToggleClass {
            Default      = "default"       ; too::views::ToggleStyle::default
            Large        = "large"         ; too::views::ToggleStyle::large
            SmallRounded = "small_rounded" ; too::views::ToggleStyle::small_rounded
            SmallDiamond = "small_diamond" ; too::views::ToggleStyle::small_diamond
            SmallSquare  = "small_square"  ; too::views::ToggleStyle::small_square
        }

        style:
        ToggleStyle => too::views::ToggleStyle{
            track             = Option<String>
            track_color       = Option<Color>
            track_hovered     = Option<Color>
            on_knob           = Option<String>
            on_knob_color     = Option<Color>
            off_knob          = Option<String>
            off_knob_color    = Option<Color>
            on_knob_hovered   = Option<Color>
            off_knob_hovered  = Option<Color>
        }
    }
}
