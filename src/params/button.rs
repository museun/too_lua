use super::Color;
use too::view::{Palette, Style, StyleOptions};

make_proxy! {
    ButtonParams {
        class:
        ButtonClass is "Button" {
            /// The default style
            Default = "default" ; too::views::ButtonStyle::default
            /// Denotes this button is for success
            Success = "success" ; too::views::ButtonStyle::success
            /// Denotes this button is for information
            Info    = "info"    ; too::views::ButtonStyle::info
            /// Denotes this button is for warning
            Warning = "warning" ; too::views::ButtonStyle::warning
            /// Denotes this button is for danger
            Danger  = "danger"  ; too::views::ButtonStyle::danger
        }

        style:
        ButtonStyle => too::views::ButtonStyle {
            /// The button text color
            text_color = Option<Color> ; "Color?"
            /// The button background color
            background = Option<Color> ; "Color?"
        }
    }
}
