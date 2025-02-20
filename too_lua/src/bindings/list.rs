use anno_lua::Anno;
use too::view::Ui;

use super::{Axis, CrossAlign, Justify};
use crate::{Context, Mapping};

#[derive(Copy, Clone, Debug, PartialEq, Anno, Default, serde::Deserialize)]
#[anno(exact)]
pub struct ListParams {
    /// Axis for the list
    #[anno(lua_type = "Axis?")]
    pub axis: Option<Axis>,

    /// Justification for children on the vertical axis
    #[anno(lua_type = "Justify?")]
    pub justify: Option<Justify>,

    /// Alignment for children on the horizontal axis
    #[anno(lua_type = "CrossAlign?")]
    pub cross_align: Option<CrossAlign>,

    /// Gap between children
    #[anno(lua_type = "integer?")]
    pub gap: Option<u16>,

    /// Should this be scrollable?
    #[anno(lua_type = "boolean?")]
    pub scrollable: Option<bool>,
}

pub fn list(mapping: &Mapping, ui: &Ui, ctx: Context, axis: Axis) {
    let params = ctx.params_de::<ListParams>().unwrap_or_default();

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
