pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// Border to surround its children
    Border is "border" => "border" {
        /// The style of the border
        style "BorderStyle?"
        /// The class of the border
        class "Border?"
        /// The border to use
        border "BorderKind"
    }
}
