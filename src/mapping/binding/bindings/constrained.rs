pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// Specifically constrain a view
    Constrained is "constrained" => "constrained" {
        /// The constraint to use
        constraint "Constraint"
    }
}
