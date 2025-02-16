pub use crate::mapping::Binding;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Separator;

impl Separator {
    binding! {
        /// Separator to divide some area
        "separator" => { }
    }
}
