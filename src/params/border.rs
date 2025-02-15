use too::view::{Palette, Style, StyleOptions};

use super::Color;
use crate::make_enum;

make_enum! {
    Border {
        Empty     = "empty"
        Thin      = "thin"
        ThinWide  = "thin_wide"
        Rounded   = "rounded"
        Double    = "double"
        Thick     = "thick"
        ThickTall = "thick_tall"
        ThickWide = "thick_wide"
    }
}

make_proxy! {
    BorderParams {
        class:
        BorderClass {
            Default      = "default"     ; too::views::BorderStyle::default
            Interactive  = "interactive" ; too::views::BorderStyle::interactive
        }

        style:
        BorderStyle => too::views::BorderStyle {
            title          = Option<Color>
            border         = Option<Color>
            border_focused = Option<Color>
            border_hovered = Option<Color>
        }
    }
}
