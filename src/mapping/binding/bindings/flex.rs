pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// Give a flex constraint to its children
    Flex is "flex" => "flex" {
        /// Tight constraint (ratio between 0.0 and 1.0)
        tight "number?"
        /// Loose constraint (ratio between 0.0 and 1.0)
        loose "number?"
    }
}
