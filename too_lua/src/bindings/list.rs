use too::{layout::Axis, view::Ui};

use crate::{Context, Mapping};

use super::{CrossAlign, Justify};

pub fn list(mapping: &Mapping, ui: &Ui, ctx: Context, axis: Axis) {
    let mut list = too::views::list().axis(axis);

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
