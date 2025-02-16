pub use crate::mapping::Binding;

pub struct Container;

impl Container {
    binding! {
        /// A container that just groups multiple calls into one parent
        "container" => "any" { }
    }
}
