use super::Color;
use too::view::{Palette, Style, StyleOptions};

make_proxy! {
    CheckboxParams {
        class:
        CheckboxClass is "Checkbox" {
            /// The default style
            Default  = "default"  ; too::views::CheckboxStyle::default
            /// A markdown inspired style
            Markdown = "markdown" ; too::views::CheckboxStyle::markdown
            /// An ascii checkbox style
            Ascii    = "ascii"    ; too::views::CheckboxStyle::ascii
        }

        style:
        CheckboxStyle => too::views::CheckboxStyle {
            /// The character to use when checked
            checked       = Option<String> ; "string?"
            /// The character to use when unchecked
            unchecked     = Option<String> ; "string?"
            /// The color of the text
            text_color    = Option<Color>  ; "Color?"
            /// The color of the text, when hovered
            hovered_color = Option<Color>  ; "Color?"
        }
    }
}
