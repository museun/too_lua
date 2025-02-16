use too::view::{Palette, Style, StyleOptions};

use super::Color;
use crate::make_enum;

make_enum! {
    enum Border is "BorderKind" {
        /// No border
        Empty     = "empty"
        /// A thin border
        Thin      = "thin"
        /// A thin, but wide border
        ThinWide  = "thin_wide"
        /// A rounded border
        Rounded   = "rounded"
        /// A double-line border
        Double    = "double"
        /// A thick border
        Thick     = "thick"
        /// A thick, but tall border
        ThickTall = "thick_tall"
        /// A thick, but wide border
        ThickWide = "thick_wide"
    }
}

make_proxy! {
    BorderParams {
        class:
        BorderClass is "Border" {
            /// The default style
            Default      = "default"     ; too::views::BorderStyle::default
            /// An interactive style
            Interactive  = "interactive" ; too::views::BorderStyle::interactive
        }

        style:
        BorderStyle => too::views::BorderStyle {
            /// The frame title color
            title          = Option<Color> ; "Color?"
            /// The color of the border
            border         = Option<Color> ; "Color?"
            /// The color of the border, when focused
            border_focused = Option<Color> ; "Color?"
            /// The color of the border, when hovered
            border_hovered = Option<Color> ; "Color?"
        }
    }
}
