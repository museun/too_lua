pub use crate::mapping::Binding;

pub struct Center;

impl Center {
    binding! {
        /// Center a view in the current layout
        "center" => "any" { }
    }
}
