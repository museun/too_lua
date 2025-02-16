pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// A button to click
    Button is "button" => "button" {
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
