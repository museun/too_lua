pub use crate::mapping::{Binding, Field};
pub struct Flex;
impl Flex {
    binding! {
        /// Give a flex constraint to its children
        "flex" => "flex" {
            /// Tight constraint (ratio between 0.0 and 1.0)
            tight "number?"
            /// Loose constraint (ratio between 0.0 and 1.0)
            loose "number?"
        }
    }
}
