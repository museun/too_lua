pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// A progress bar
    Progress is "progress" => "Value | progress" {
        /// The style of the progress bar
        style "ProgressStyle?"
        /// The class of the progress bar
        class "Progress?"
        /// Axis to use for layout
        axis "Axis?"
        /// The value to use (an f32 in the range of 0.0 ..= 1.0)
        value "Value"
    }
}
