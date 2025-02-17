use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

use super::{Color, Value};

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Checkbox;

impl Checkbox {
    binding! {
        /// A checkbox to toggle a boolean
        "checkbox" => "checkbox" {
            /// The style of the checkbox
            style "CheckboxStyle?"
            /// The class of the checkbox
            class "Checkbox?"
            /// The text of the checkbox
            text "string | lazy_args"
            /// The state of the checkbox, a boolean
            value "Value"
        }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<CheckboxParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "checkbox", "params");
        };

        let Some(Ok(text)) = ctx.params_field::<String>("text") else {
            return Mapping::report_missing_data(ui, ctx.id, "checkbox", "text");
        };

        let Some(mut value) = ctx.value_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "checkbox", "value");
        };

        let Value::Bool(value) = &mut *value else {
            return Mapping::report_missing(ui, ctx.id, "bool value");
        };

        ui.show(too::views::checkbox(value, text).class(params.apply()));
    }
}
