use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Center;

impl BindingView for Center {
    const SPEC: BindingSpec = binding! {
        /// Center aligns its children
        "center" => any
    };

    type Params = ();
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        ui.center(|ui| ctx.visit_children(mapping, ui));
    }
}
