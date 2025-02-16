use too::{layout::Axis, view::Ui};

use crate::{params, Context, Mapping};

pub fn list(mapping: &Mapping, ui: &Ui, ctx: Context, axis: Axis) {
    let mut list = too::views::list().axis(axis);

    if let Some(justify) = ctx.params_field_opt::<params::Justify>("justify") {
        let justify = match justify {
            params::Justify::Start => too::layout::Justify::Start,
            params::Justify::End => too::layout::Justify::End,
            params::Justify::Center => too::layout::Justify::Center,
            params::Justify::SpaceBetween => too::layout::Justify::SpaceBetween,
            params::Justify::SpaceAround => too::layout::Justify::SpaceAround,
            params::Justify::SpaceEvenly => too::layout::Justify::SpaceEvenly,
        };
        list = list.justify(justify);
    }

    if let Some(cross_align) = ctx.params_field_opt::<params::CrossAlign>("cross_align") {
        let cross_align = match cross_align {
            params::CrossAlign::Start => too::layout::CrossAlign::Start,
            params::CrossAlign::End => too::layout::CrossAlign::End,
            params::CrossAlign::Center => too::layout::CrossAlign::Center,
            params::CrossAlign::Stretch => too::layout::CrossAlign::Stretch,
            params::CrossAlign::Fill => too::layout::CrossAlign::Fill,
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
