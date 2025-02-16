pub use crate::mapping::{Binding, Field};
pub struct ToggleSwitch;
impl ToggleSwitch {
    binding! {
        /// A switch that is toggled when clicked
        "toggle_switch" => "toggle_switch" {
            /// The style of the selected value
            style "ToggleStyle?"
            /// The class of the selected value
            class "Toggle?"
            /// The state of the selected value, a boolean
            value "Value"
        }
    }
}
