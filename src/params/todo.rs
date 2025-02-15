use super::Color;

make_proxy! {
    TodoParams {
        class:
        TodoClass {
            Default  = "default" ; too::views::TodoStyle::default
        }

        manual style:
        TodoStyle => too::views::TodoStyle {
            bold          = Option<bool>
            faint         = Option<bool>
            italic        = Option<bool>
            underline     = Option<bool>
            blink         = Option<bool>
            reverse       = Option<bool>
            strikeout     = Option<bool>
            //
            text_color    = Option<Color>
            hovered_color = Option<Color>
        }
    }
}
