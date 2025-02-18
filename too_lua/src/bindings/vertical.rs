use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, LuaType, Mapping,
};

use super::ListParams;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertical;

impl BindingView for Vertical {
    const SPEC: BindingSpec = binding! {
        /// Vertical layout of children
        "vertical" => ListParams::NAME
    };

    type Params = ListParams;
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        super::list::list(mapping, ui, ctx, super::Axis::Vertical);
    }
}
