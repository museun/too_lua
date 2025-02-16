pub use crate::mapping::{Binding, Field};

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
            text "string"
            /// The state of the checkbox, a boolean
            value "Value"
        }
    }
}
