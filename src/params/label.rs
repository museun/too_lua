use super::Color;

make_proxy! {
    LabelParams {
        class:
        LabelClass {
            Default = "default" ; too::views::LabelStyle::default
            Info    = "info"    ; too::views::LabelStyle::info
            Warning = "warning" ; too::views::LabelStyle::warning
            Success = "success" ; too::views::LabelStyle::success
            Danger  = "danger"  ; too::views::LabelStyle::danger
        }

        manual style:
        LabelStyle => too::views::LabelStyle {
            foreground = Option<Color>
            italic     = Option<bool>
            bold       = Option<bool>
            underline  = Option<bool>
            faint      = Option<bool>
            blink      = Option<bool>
            strikeout  = Option<bool>
        }
    }
}
