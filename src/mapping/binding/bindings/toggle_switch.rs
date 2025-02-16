pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// A switch that is toggled when clicked
    ToggleSwitch is "toggle_switch" => "toggle_switch" {
        /// The style of the selected value
        style "ToggleStyle?"
        /// The class of the selected value
        class "Toggle?"
        /// The state of the selected value, a boolean
        value "Value"
    }
}
