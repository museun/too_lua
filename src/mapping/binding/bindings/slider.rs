pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// A slider to adjust a value
    Slider is "slider" => "Value | slider" {
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
