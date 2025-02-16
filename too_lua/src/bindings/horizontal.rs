use too::{layout::Axis, view::Ui};

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Horizontal;

impl Horizontal {
    binding! {
        /// Horizontal layout of children
        "horizontal" => "horizontal" {
            // TODO this can be styled
            /// Justification for children on the horizontal axis
            justify "Justify?"
            /// Alignment for children on the vertical axis
            cross_align "CrossAlign?"
            /// Gap between children
            gap "integer?"
            /// Should this be scrollable?
            scrollable "boolean?"
        }
    }

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        super::list::list(mapping, ui, ctx, Axis::Horizontal);
    }
}
