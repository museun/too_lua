pub use crate::mapping::{Binding, Field};

pub struct Progress;
impl Progress {
    binding! {
        /// A progress bar
        "progress" => "Value | progress" {
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
}
