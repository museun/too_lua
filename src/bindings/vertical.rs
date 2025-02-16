pub use crate::mapping::{Binding, Field};

pub struct Vertical;
impl Vertical {
    binding! {
        /// Vertical layout of children
        "vertical" => "vertical" {
          /// Justification for children on the vertical axis
          justify "Justify?"
          /// Alignment for children on the horizontal axis
          cross_align "CrossAlign?"
          /// Gap between children
          gap "integer?"
          /// Should this be scrollable?
          scrollable "boolean?"
        }
    }
}
