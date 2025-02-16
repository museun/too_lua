pub use crate::mapping::{Binding, Field};

pub struct Fill;

impl Fill {
    binding! {
        /// Fill the childrens area, with an optional size constraint
        "fill" => "fill" {
            /// Use this color to fill the area
            background "string"
            /// Optional space to allocate
            space "{width: integer?, height: integer?}"
        }
    }
}
