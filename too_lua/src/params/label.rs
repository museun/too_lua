use super::Color;

make_proxy! {
    LabelParams {
        class:
        LabelClass is "Label" {
            /// The default style
            Default = "default" ; too::views::LabelStyle::default
            /// Denotes this label is for information
            Info    = "info"    ; too::views::LabelStyle::info
            /// Denotes this label is for warning
            Warning = "warning" ; too::views::LabelStyle::warning
            /// Denotes this label is for success
            Success = "success" ; too::views::LabelStyle::success
            /// Denotes this label is for danger
            Danger  = "danger"  ; too::views::LabelStyle::danger
        }

        manual style:
        LabelStyle => too::views::LabelStyle {
            /// The foreground text color
            foreground = Option<Color> ; "Color?"
            /// The text should be italic
            italic     = Option<bool>  ; "boolean?"
            /// The text should be bold
            bold       = Option<bool>  ; "boolean?"
            /// The text should be underline
            underline  = Option<bool>  ; "boolean?"
            /// The text should be faint
            faint      = Option<bool>  ; "boolean?"
            /// The text should be blink
            blink      = Option<bool>  ; "boolean?"
            /// The text should be strikeout
            strikeout  = Option<bool>  ; "boolean?"
        }
    }
}
