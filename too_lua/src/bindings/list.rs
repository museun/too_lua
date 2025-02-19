use too::view::Ui;

use super::{Axis, CrossAlign, Justify};
use crate::{Context, Mapping};

make_struct! {
    struct ListParams is "ListParams" {
        /// Axis for the list
        axis        = Option<Axis>       ; "Axis?"
        /// Justification for children on the vertical axis
        justify     = Option<Justify>    ; "Justify?"
        /// Alignment for children on the horizontal axis
        cross_align = Option<CrossAlign> ; "CrossAlign?"
        /// Gap between children
        gap         = Option<u16>        ; "integer?"
        /// Should this be scrollable?
        scrollable  = Option<bool>       ; "boolean?"
    }
}

impl Default for ListParams {
    fn default() -> Self {
        Self {
            axis: None,
            justify: None,
            cross_align: None,
            gap: None,
            scrollable: None,
        }
    }
}

pub fn list(mapping: &Mapping, ui: &Ui, ctx: Context, axis: Axis) {
    let params = ctx.params::<ListParams>().unwrap_or_default();

    let list = too::views::list()
        .axis(params.axis.unwrap_or(axis).into())
        .justify(params.justify.unwrap_or_default().into())
        .cross_align(params.cross_align.unwrap_or_default().into())
        .gap(params.gap.unwrap_or(match axis {
            Axis::Vertical => 0,
            Axis::Horizontal => 1,
        }) as i32)
        .scrollable(params.scrollable.unwrap_or_default());

    ui.show_children(list, |ui| ctx.visit_children(mapping, ui));
}
