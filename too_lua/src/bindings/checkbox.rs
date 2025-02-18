use mlua::AnyUserData;
use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    mapping::{BindingSpec, BindingView},
    proxy::Params,
    Context, Mapping,
};

use super::Color;

make_class! {
    class CheckboxClass is "Checkbox" ; too::views::CheckboxStyle {
        /// The default style
        Default  = "default"  ; too::views::CheckboxStyle::default
        /// A markdown inspired style
        Markdown = "markdown" ; too::views::CheckboxStyle::markdown
        /// An ascii checkbox style
        Ascii    = "ascii"    ; too::views::CheckboxStyle::ascii
    }
}

make_style! {
    style CheckboxStyle is "CheckboxStyle" ; too::views::CheckboxStyle {
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

make_struct! {
    struct CheckboxParams is "CheckboxParams" {
        /// The style of the checkbox
        style  = Option<CheckboxStyle> ; "CheckboxStyle?"
        /// The class of the checkbox
        class  = Option<CheckboxClass> ; "Checkbox?"
        /// The text of the checkbox
        text   = String                ; "string"
        /// The state of the checkbox, a boolean
        value  = AnyUserData           ; "Value"
    }
}

impl Params<too::views::CheckboxStyle> for CheckboxParams {
    type Class = CheckboxClass;
    type Style = CheckboxStyle;

    fn class(&self) -> &Option<Self::Class> {
        &self.class
    }
    fn style(&self) -> &Option<Self::Style> {
        &self.style
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Checkbox;

impl BindingView for Checkbox {
    const SPEC: BindingSpec = binding! {
        /// A checkbox to toggle a boolean
        "checkbox" => "CheckboxParams"
    };

    type Params = CheckboxParams;
    type Style = CheckboxStyle;

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.foo::<CheckboxParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "checkbox", "params");
        };

        let Some(mut value) = ctx.value_mut(&params.value) else {
            return Mapping::report_missing_data(ui, ctx.id, "checkbox", "value");
        };

        let Some(value) = value.bool_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "checkbox", "bool");
        };

        ui.show(
            too::views::checkbox(
                value, //
                &params.text,
            )
            .class(params.apply_styling()),
        );
    }
}
