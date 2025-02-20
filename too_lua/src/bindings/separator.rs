use too::view::Ui;

use crate::{Context, Mapping, None, Spec, View};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Separator;

impl View for Separator {
    type Params = None;
    type Style = None;

    fn spec() -> Spec {
        view_spec! {
            /// Separator to divide some area
            Self {
                name: "separator"
            }
        }
    }

    fn view(_mapping: &Mapping, ui: &Ui, _ctx: Context) {
        // TODO this can be styled
        ui.separator();
    }
}
