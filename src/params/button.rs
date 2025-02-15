use super::Color;
use too::view::{Palette, Style, StyleOptions};

make_proxy! {
    ButtonParams {
        class:
        ButtonClass {
            Default = "default" ; too::views::ButtonStyle::default
            Success = "success" ; too::views::ButtonStyle::success
            Info    = "info"    ; too::views::ButtonStyle::info
            Warning = "warning" ; too::views::ButtonStyle::warning
            Danger  = "danger"  ; too::views::ButtonStyle::danger
        }

        style:
        ButtonStyle => too::views::ButtonStyle {
            text_color = Option<Color>
            background = Option<Color>
        }
    }
}
