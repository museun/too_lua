pub use crate::mapping::binding::{Binding, Field};

binding! {
    /// Frame is a border with a title
    Frame is "frame" => "frame" {
       /// The style of the frame
       style "BorderStyle?"
       /// The class of the frame
       class "Border?"
       /// The border to use
       border "BorderKind"
       /// Alignment for the title
       align "Align?"
       /// A string to place in the title
       title "string"
    }
}
