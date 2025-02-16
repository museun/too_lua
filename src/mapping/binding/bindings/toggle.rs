pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// Conditionally show or hide a view
    Toggle is "toggle" => "toggle" {
        /// The boolean state to use
        value "Value"
    }
}
