pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// Margin applies padding to a view
    Margin is "margin" => "margin" {
        /// Padding to the left of the view
        left "integer?"
        /// Padding to the right of the view
        right "integer?"
        /// Padding to the top of the view
        top "integer?"
        /// Padding to the bottom of the view
        bottom "integer?"
        /// Padding on both left and right of the view
        horizontal "integer?"
        /// Padding on both top and bottom of the view
        vertical "integer?"
        /// Padding on each side of the view
        all "integer?"
    }
}
