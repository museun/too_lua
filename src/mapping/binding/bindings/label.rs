pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// Label displays some text
    Label is "label" => "string | label" {
        /// The style of the label
        style "LabelStyle?"
        /// The class of the label
        class "Label?"
        /// The text of the label
        text "string"
    }
}
