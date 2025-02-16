pub use crate::mapping::Binding;

pub struct ExpandAxis;
impl ExpandAxis {
    binding! {
        /// A view that expands the remainder of the space on the axis
        "expand_axis" => { }
    }
}
