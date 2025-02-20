use too::view::Ui;

use crate::{Context, Mapping, None, Spec, View};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Container;

impl View for Container {
    type Params = None;
    type Style = None;

    fn spec() -> Spec {
        view_spec! {
            /// A container that just groups multiple calls into one parent
            Self {
                name: "container",
                params: any
            }
        }
    }

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        ctx.visit_children(mapping, ui);
    }
}
