pub use crate::mapping::{Binding, Field};

pub struct Label;
impl Label {
    binding! {
        /// Label displays some text
        "label" => "string | label" {
            /// The style of the label
            style "LabelStyle?"
            /// The class of the label
            class "Label?"
            /// The text of the label
            text "string"
        }
    }
}
