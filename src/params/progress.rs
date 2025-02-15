use super::Color;
use too::view::{Palette, Style, StyleOptions};

make_proxy! {
    ProgressParams {
        class:
        ProgressClass {
            Default      = "default"       ; too::views::ProgressStyle::default
            MediumFilled = "medium_filled" ; too::views::ProgressStyle::medium_filled
            Filled       = "filled"        ; too::views::ProgressStyle::filled
            Thin         = "thin"          ; too::views::ProgressStyle::thin
            Thick        = "thick"         ; too::views::ProgressStyle::thick
            ThinDashed   = "thin_dashed"   ; too::views::ProgressStyle::thin_dashed
            ThickDashed  = "thick_dashed"  ; too::views::ProgressStyle::thick_dashed
        }

        style:
        ProgressStyle => too::views::ProgressStyle {
            unfilled_color   = Option<Color>
            filled_color     = Option<Color>
            unfilled_hovered = Option<Color>
            filled_hovered   = Option<Color>
            unfilled         = Option<String>
            filled           = Option<String>
        }
    }
}
