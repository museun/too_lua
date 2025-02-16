pub use crate::mapping::{Binding, Field};
pub struct Toggle;
impl Toggle {
    binding! {
        /// Conditionally show or hide a view
        "toggle" => "toggle" {
            /// The boolean state to use
            value "Value"
        }
    }
}
