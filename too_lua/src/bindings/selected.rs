use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

use super::{Color, Value};

make_proxy! {
    SelectedParams {
        class:
        SelectedClass is "Selected" {
            /// The default style
            Default  = "default" ; too::views::SelectedStyle::default
            /// This element reacts to hovers
            Hovered  = "hovered" ; too::views::SelectedStyle::hovered
        }

        style:
        SelectedStyle => too::views::SelectedStyle {
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
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Selected;

impl Selected {
    binding! {
        /// A selected boolean value
        "selected" => "selected" {
          /// The style of the selected value
          style "SelectedStyle?"
          /// The class of the selected value
          class "Selected?"
          /// The text of the selected value
          text "string"
          /// The state of the selected value, a boolean
          value "Value"
        }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
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

        ui.show(too::views::selected(value, text).class(params.apply()));
    }
}
