use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

use super::Color;

crate::make_proxy! {
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Button;

impl Button {
    binding! {
        /// A button to click
        "button" => "button" {
            /// The style of the button
            style "ButtonStyle?"
            /// The class of the button
            class "Button?"
            /// The text of the button
            text "string | lazy_args"
            /// Function to call when the button is clicked
            handler "fun(): nil"
        }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<ButtonParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "button", "params");
        };

        let Some(Ok(text)) = ctx.params_field::<String>("text") else {
            return Mapping::report_missing_data(ui, ctx.id, "button", "text");
        };

        let Some(Ok(handler)) = ctx.params_field::<mlua::Function>("handler") else {
            return Mapping::report_missing_data(ui, ctx.id, "button", "handler");
        };

        let view = too::views::button(text).class(params.apply());

        if ui.show(view).clicked() {
            let _ = handler.call::<()>(());
        }
    }
}
