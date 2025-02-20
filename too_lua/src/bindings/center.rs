use too::view::Ui;

use crate::{Context, Mapping, None, Spec, View};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Center;

impl View for Center {
    type Params = None;
    type Style = None;

    fn spec() -> Spec {
        view_spec! {
            /// Center aligns its children
            Self {
                name: "center",
                params: any
            }
        }
    }

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        ui.center(|ui| ctx.visit_children(mapping, ui));
    }
}
