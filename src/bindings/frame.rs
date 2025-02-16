pub use crate::mapping::{Binding, Field};

pub struct Frame;
impl Frame {
    binding! {
        /// Frame is a border with a title
        "frame" => "frame" {
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
}
