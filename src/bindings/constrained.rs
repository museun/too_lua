pub use crate::mapping::{Binding, Field};

pub struct Constrained;

impl Constrained {
    binding! {
        /// Specifically constrain a view
        "constrained" => "constrained" {
            /// The constraint to use
            constraint "Constraint"
        }
    }
}
