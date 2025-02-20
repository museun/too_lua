use too::view::Ui;

use crate::{Context, Mapping, None, Spec, View};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExpandAxis;

impl View for ExpandAxis {
    type Params = None;
    type Style = None;

    fn spec() -> Spec {
        view_spec! {
            /// A view that expands the remainder of the space on the axis
            Self {
                name: "expand_axis"
            }
        }
    }

    fn view(_mapping: &Mapping, ui: &Ui, _ctx: Context) {
        ui.expand_axis();
    }
}
