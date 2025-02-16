pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// Fill the childrens area, with an optional size constraint
    Fill is "fill" => "fill" {
        /// Use this color to fill the area
        background "string"
        /// Optional space to allocate
        space "{width: integer?, height: integer?}"
    }
}
