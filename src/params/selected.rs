use super::Color;
use too::view::{Palette, Style, StyleOptions};

make_proxy! {
    SelectedParams {
        class:
        SelectedClass is "Selected" {
            /// The default style
            Default  = "default" ; too::views::SelectedStyle::default
            /// This element reacts to hovers
            Hovered  = "hovered" ; too::views::SelectedStyle::hovered
        }

        style:
        SelectedStyle => too::views::SelectedStyle {
            /// The background color
            background          = Option<Color> ; "Color?"
            /// The text color
            text_color          = Option<Color> ; "Color?"
            /// The background color, when selected
            selected_background = Option<Color> ; "Color?"
            /// The text color, when hovered
            hovered_text        = Option<Color> ; "Color?"
            /// The background color, when hovered
            hovered_background  = Option<Color> ; "Color?"
        }
    }
}
