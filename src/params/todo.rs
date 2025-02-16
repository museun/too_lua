use super::Color;

make_proxy! {
    TodoParams {
        class:
        TodoClass is "Todo" {
            /// The default style
            Default  = "default" ; too::views::TodoStyle::default
        }

        manual style:
        TodoStyle => too::views::TodoStyle {
            /// When selected, the text should be bold
            bold          = Option<bool> ; "boolean?"
            /// When selected, the text should be faint
            faint         = Option<bool> ; "boolean?"
            /// When selected, the text should be italic
            italic        = Option<bool> ; "boolean?"
            /// When selected, the text should be underline
            underline     = Option<bool> ; "boolean?"
            /// When selected, the text should be blink
            blink         = Option<bool> ; "boolean?"
            /// When selected, the text should be reverse
            reverse       = Option<bool> ; "boolean?"
            /// When selected, the text should be strikeout
            strikeout     = Option<bool> ; "boolean?"
            //
            /// The color of the text
            text_color    = Option<Color> ; "Color?"
            /// The color of the text, when hovered
            hovered_color = Option<Color> ; "Color?"
        }
    }
}
