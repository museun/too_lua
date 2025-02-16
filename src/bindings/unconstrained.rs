pub use crate::mapping::{Binding, Field};
pub struct Unconstrained;
impl Unconstrained {
    binding! {
        /// Specifically unconstrained a view
        "unconstrained" => "unconstrained" {
            /// Which axis to remove the constraints for
            constraint "{horizontal: boolean?, vertical: boolean?, both: boolean?}"
        }
    }
}
