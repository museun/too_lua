use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExpandAxis;

impl BindingView for ExpandAxis {
    const SPEC: BindingSpec = binding! {
        /// A view that expands the remainder of the space on the axis
        "expand_axis"
    };

    type Params = ();
    type Style = ();

    fn view(_mapping: &Mapping, ui: &Ui, _ctx: Context) {
        ui.expand_axis();
    }
}
