pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// Horizontal layout of children
    Horizontal is "horizontal" => "horizontal" {
        // TODO this can be styled
        /// Justification for children on the horizontal axis
        justify "Justify?"
        /// Alignment for children on the vertical axis
        cross_align "CrossAlign?"
        /// Gap between children
        gap "integer?"
        /// Should this be scrollable?
        scrollable "boolean?"
    }
}
