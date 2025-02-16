pub use crate::mapping::{Binding, Field};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Aligned;

impl Aligned {
    binding! {
        /// Align its children at a specific anchor
        "aligned" => "aligned" {
            /// Alignment for the children
            align "Aligned"
        }
    }
}
