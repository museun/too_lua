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

pub fn list(mapping: &Mapping, ui: &Ui, ctx: Context, axis: Axis) {
    let mut list = too::views::list().axis(match axis {
        Axis::Vertical => too::layout::Axis::Vertical,
        Axis::Horizontal => too::layout::Axis::Horizontal,
    });

    if let Some(justify) = ctx.params_field_opt::<Justify>("justify") {
        let justify = match justify {
            Justify::Start => too::layout::Justify::Start,
            Justify::End => too::layout::Justify::End,
            Justify::Center => too::layout::Justify::Center,
            Justify::SpaceBetween => too::layout::Justify::SpaceBetween,
            Justify::SpaceAround => too::layout::Justify::SpaceAround,
            Justify::SpaceEvenly => too::layout::Justify::SpaceEvenly,
        };
        list = list.justify(justify);
    }

    if let Some(cross_align) = ctx.params_field_opt::<CrossAlign>("cross_align") {
        let cross_align = match cross_align {
            CrossAlign::Start => too::layout::CrossAlign::Start,
            CrossAlign::End => too::layout::CrossAlign::End,
            CrossAlign::Center => too::layout::CrossAlign::Center,
            CrossAlign::Stretch => too::layout::CrossAlign::Stretch,
            CrossAlign::Fill => too::layout::CrossAlign::Fill,
        };
        list = list.cross_align(cross_align);
    }

    if let Some(gap) = ctx.params_field_opt::<u16>("gap") {
        list = list.gap(gap as i32)
    }

    if let Some(scrollable) = ctx.params_field_opt::<bool>("scrollable") {
        list = list.scrollable(scrollable)
    }

    ui.show_children(list, |ui| ctx.visit_children(mapping, ui));
}
