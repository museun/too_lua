use too::view::Ui;

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExpandAxis;

impl ExpandAxis {
    binding! {
        /// A view that expands the remainder of the space on the axis
        "expand_axis" => { }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, _ctx: Context) {
        ui.expand_axis();
    }
}
