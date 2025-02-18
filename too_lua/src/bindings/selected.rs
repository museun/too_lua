use mlua::AnyUserData;
use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    mapping::{BindingSpec, BindingView},
    proxy::Params,
    Context, LuaType, Mapping,
};

use super::{Color, Value};

make_class! {
    class SelectedClass is "Selected" ; too::views::SelectedStyle {
        /// The default style
        Default  = "default" ; too::views::SelectedStyle::default
        /// This element reacts to hovers
        Hovered  = "hovered" ; too::views::SelectedStyle::hovered
    }
}

make_style! {
    style SelectedStyle is "SelectedStyle" ; too::views::SelectedStyle {
        /// The background color
        background          = Option<Color> ; "Color?"
        /// The text color
        text_color          = Option<Color> ; "Color?"
        /// The background color, when selected
        selected_background = Option<Color> ; "Color?"
        /// The text color, when hovered
        hovered_text        = Option<Color> ; "Color?"
        /// The background color, when hovered
        hovered_background  = Option<Color> ; "Color?"
    }
}

make_struct! {
    struct SelectedParams is "SelectedParams" {
        /// The style of the selected value
        style = Option<SelectedStyle> ; "SelectedStyle?"
        /// The class of the selected value
        class = Option<SelectedClass> ; "Selected?"
        /// The text of the selected value
        text  = String                ; "string"
        /// The state of the selected value, a boolean
        value = AnyUserData           ; "Value"
    }
}

impl Params<too::views::SelectedStyle> for SelectedParams {
    type Class = SelectedClass;
    type Style = SelectedStyle;

    fn class(&self) -> &Option<Self::Class> {
        &self.class
    }
    fn style(&self) -> &Option<Self::Style> {
        &self.style
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Selected;

impl BindingView for Selected {
    const SPEC: BindingSpec = binding! {
        /// A selected boolean value
        "selected" => SelectedParams::NAME
    };
    type Params = SelectedParams;
    type Style = SelectedStyle;

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<SelectedParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "selected", "params");
        };

        let Some(Ok(text)) = ctx.params_field::<String>("text") else {
            return Mapping::report_missing_data(ui, ctx.id, "selected", "text");
        };

        let Some(mut value) = ctx.value_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "selected", "value");
        };

        let Value::Bool(value) = &mut *value else {
            return Mapping::report_missing(ui, ctx.id, "bool value");
        };

        ui.show(too::views::selected(value, text).class(params.apply_styling()));
    }
}
