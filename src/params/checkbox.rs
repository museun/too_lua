use super::Color;
use too::view::{Palette, Style, StyleOptions};

make_proxy! {
    CheckboxParams {
        class:
        CheckboxClass {
            Default  = "default"  ; too::views::CheckboxStyle::default
            Markdown = "markdown" ; too::views::CheckboxStyle::markdown
            Ascii    = "ascii"    ; too::views::CheckboxStyle::ascii
        }

        style:
        CheckboxStyle => too::views::CheckboxStyle {
            checked = Option<String>
            unchecked = Option<String>
            text_color = Option<Color>
            hovered_color = Option<Color>
        }
    }
}
