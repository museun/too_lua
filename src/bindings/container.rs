use too::view::Ui;

use crate::{mapping::Binding, Context, Mapping};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Container;

impl Container {
    binding! {
        /// A container that just groups multiple calls into one parent
        "container" => "any" { }
    }

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        ctx.visit_children(mapping, ui);
    }
}
