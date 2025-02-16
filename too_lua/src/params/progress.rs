use super::Color;
use too::view::{Palette, Style, StyleOptions};

make_proxy! {
    ProgressParams {
        class:
        ProgressClass is "Progress" {
            /// Default style
            Default      = "default"       ; too::views::ProgressStyle::default
            /// A medium filled style
            MediumFilled = "medium_filled" ; too::views::ProgressStyle::medium_filled
            /// A full filled style
            Filled       = "filled"        ; too::views::ProgressStyle::filled
            /// A thin style
            Thin         = "thin"          ; too::views::ProgressStyle::thin
            /// A thick style
            Thick        = "thick"         ; too::views::ProgressStyle::thick
            /// A thin, but dashed style
            ThinDashed   = "thin_dashed"   ; too::views::ProgressStyle::thin_dashed
            /// A thick, but dashed style
            ThickDashed  = "thick_dashed"  ; too::views::ProgressStyle::thick_dashed
        }

        style:
        ProgressStyle => too::views::ProgressStyle {
            /// The unfilled color
            unfilled_color   = Option<Color>  ; "Color?"
            /// The filled color
            filled_color     = Option<Color>  ; "Color?"
            /// The unfilled color, when hovered
            unfilled_hovered = Option<Color>  ; "Color?"
            /// The filled color, when hovered
            filled_hovered   = Option<Color>  ; "Color?"
            /// The character to use for the unfilled portion
            unfilled         = Option<String> ; "string?"
            /// The character to use for the filled portion
            filled           = Option<String> ; "string?"
        }
    }
}
