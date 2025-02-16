pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// Specifically unconstrained a view
    Unconstrained is "unconstrained" => "unconstrained" {
        /// Which axis to remove the constraints for
        constraint "{horizontal: boolean?, vertical: boolean?, both: boolean?}"
    }
}
