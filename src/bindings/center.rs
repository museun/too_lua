use too::view::Ui;

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Center;

impl Center {
    binding! {
        /// Center a view in the current layout
        "center" => "any" { }
    }

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        ui.center(|ui| ctx.visit_children(mapping, ui));
    }
}
