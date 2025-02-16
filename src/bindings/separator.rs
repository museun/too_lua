use too::view::Ui;

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Separator;

impl Separator {
    binding! {
        /// Separator to divide some area
        "separator" => { }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, _ctx: Context) {
        // TODO this can be styled
        ui.separator();
    }
}
