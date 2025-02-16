pub use crate::mapping::{Binding, Field};

pub struct Slider;
impl Slider {
    binding! {
        /// A slider to adjust a value
        "slider" => "Value | slider" {
            /// The style of the slider
            style "SliderStyle?"
            /// The class of the slider
            class "Slider?"
            /// Axis to use for layout
            axis "Axis?"
            /// The value to use (an f32 in the range of 0.0 ..= 1.0)
            value "Value"
        }
    }
}
