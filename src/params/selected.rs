use super::Color;
use too::view::{Palette, Style, StyleOptions};

make_proxy! {
    SelectedParams {
        class:
        SelectedClass {
            Default  = "default" ; too::views::SelectedStyle::default
            Hovered  = "hovered" ; too::views::SelectedStyle::hovered
        }

        style:
        SelectedStyle => too::views::SelectedStyle {
            background = Option<Color>
            text_color = Option<Color>
            selected_background = Option<Color>
            hovered_text = Option<Color>
            hovered_background = Option<Color>
        }
    }
}
