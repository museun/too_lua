use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Container;

impl BindingView for Container {
    const SPEC: BindingSpec = binding! {
        /// "A container that just groups multiple calls into one parent"
        "container" => any
    };

    type Params = ();
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        ctx.visit_children(mapping, ui);
    }
}
