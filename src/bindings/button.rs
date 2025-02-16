use too::view::{Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    params, Context, Mapping,
};

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
            text "string"
            /// Function to call when the button is clicked
            handler "fun(): nil"
        }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<params::ButtonParams>() else {
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
