use too::view::Ui;

use crate::{Context, Mapping, None, Spec, View};

use super::ListParams;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Horizontal;

impl View for Horizontal {
    type Params = ListParams;
    // TODO styling
    type Style = None;

    fn spec() -> Spec {
        view_spec! {
            /// Horizontal layout of children
            Self {
                name: "horizontal",
                params: "ListParams"
            }
        }
    }

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        super::list::list(mapping, ui, ctx, super::Axis::Horizontal);
    }
}
