pub use crate::mapping::{Binding, Field};

pub struct Button;
impl Button {
    binding! {
        /// A button to click
        "button" => "button" {
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
}
