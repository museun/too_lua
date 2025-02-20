use too::view::Ui;

use crate::{Context, Mapping, None, Spec, View};

use super::ListParams;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertical;

impl View for Vertical {
    type Params = ListParams;
    // TODO styling
    type Style = None;

    fn spec() -> Spec {
        view_spec! {
            /// Vertical layout of children
            Self {
                name: "vertical",
                params: "ListParams"
            }
        }
    }

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        super::list::list(mapping, ui, ctx, super::Axis::Vertical);
    }
}
