use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, LuaType, Mapping,
};

use super::ListParams;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Horizontal;

impl BindingView for Horizontal {
    const SPEC: BindingSpec = binding! {
        /// Horizontal layout of children
        "horizontal" => ListParams::NAME
    };

    type Params = ListParams;
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        super::list::list(mapping, ui, ctx, super::Axis::Horizontal);
    }
}
