pub use crate::mapping::{Binding, Field};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Background;

impl Background {
    binding! {
        /// Background of its children
        "background" => "background" {
            /// The background color for the children
            background "string"
        }
    }
}
