use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    mapping::{BindingSpec, BindingView},
    proxy::Params,
    Context, LuaType, Mapping,
};

use super::Color;

make_class! {
    class ButtonClass is "Button" ; too::views::ButtonStyle {
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
}

make_style! {
    style ButtonStyle is "ButtonStyle" ; too::views::ButtonStyle {
        /// The button text color
        text_color = Option<Color> ; "Color?"
        /// The button background color
        background = Option<Color> ; "Color?"
    }
}

make_struct! {
    struct ButtonParams is "ButtonParams" {
        /// The style of the button
        style   = Option<ButtonStyle> ; "ButtonStyle?"
        /// The class of the button
        class   = Option<ButtonClass> ; "Button?"
        /// The text of the button
        text    = String              ; "string"
        /// Function to call when the button is clicked
        handler = mlua::Function      ; "fun(): nil"
    }
}

impl Params<too::views::ButtonStyle> for ButtonParams {
    type Class = ButtonClass;
    type Style = ButtonStyle;

    fn class(&self) -> &Option<Self::Class> {
        &self.class
    }
    fn style(&self) -> &Option<Self::Style> {
        &self.style
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Button;

impl BindingView for Button {
    const SPEC: BindingSpec = binding! {
        /// A button to click
        "button" => ButtonParams::NAME
    };

    type Params = ButtonParams;
    type Style = ButtonStyle;

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<ButtonParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "button", "params");
        };

        let Some(Ok(text)) = ctx.params_field::<String>("text") else {
            return Mapping::report_missing_data(ui, ctx.id, "button", "text");
        };

        let Some(Ok(handler)) = ctx.params_field::<mlua::Function>("handler") else {
            return Mapping::report_missing_data(ui, ctx.id, "button", "handler");
        };

        let view = too::views::button(text).class(params.apply_styling());

        if ui.show(view).clicked() {
            let _ = handler.call::<()>(());
        }
    }
}
