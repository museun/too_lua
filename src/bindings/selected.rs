pub use crate::mapping::{Binding, Field};
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
}
