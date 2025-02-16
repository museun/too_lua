pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// Background of its children
    Background is "background" => "background" {
        /// The background color for the children
        background "string"
    }
}
