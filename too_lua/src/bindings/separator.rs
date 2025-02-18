use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Separator;

impl BindingView for Separator {
    const SPEC: BindingSpec = binding! {
        /// Separator to divide some area
        "separator"
    };

    type Params = ();
    type Style = ();

    fn view(_mapping: &Mapping, ui: &Ui, _ctx: Context) {
        // TODO this can be styled
        ui.separator();
    }
}
