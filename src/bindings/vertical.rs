use too::{layout::Axis, view::Ui};

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
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

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        super::list::list(mapping, ui, ctx, Axis::Vertical);
    }
}
