pub use crate::mapping::{Binding, Field};

pub struct Border;

impl Border {
    binding! {
        /// Border to surround its children
        "border" => "border" {
            /// The style of the border
            style "BorderStyle?"
            /// The class of the border
            class "Border?"
            /// The border to use
            border "BorderKind"
        }
    }
}
