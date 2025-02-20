use anno_lua::Anno;
use too::view::Ui;

use crate::{Context, Mapping, None, Spec, View};

#[derive(Copy, Clone, Debug, PartialEq, Anno, serde::Deserialize)]
#[anno(exact)]
pub struct UnconstrainedParams {
    /// Unconstrain the horizontal axis
    #[anno(lua_type = "boolean?")]
    pub horizontal: Option<bool>,

    /// Unconstrain the vertical axis
    #[anno(lua_type = "boolean?")]
    pub vertical: Option<bool>,

    /// Unconstrain both axis
    #[anno(lua_type = "boolean?")]
    pub both: Option<bool>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Unconstrained;

impl View for Unconstrained {
    type Params = UnconstrainedParams;
    type Style = None;

    fn spec() -> Spec {
        view_spec! {
            /// Specifically unconstrained a view
            Self {
                name: "unconstrained",
                params: "UnconstrainedParams"
            }
        }
    }

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params_de::<UnconstrainedParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "unconstrained", "params");
        };

        let both = params.both.unwrap_or_default();
        let horizontal = params.horizontal.unwrap_or_default();
        let vertical = params.vertical.unwrap_or_default();

        let view = if both {
            too::views::Unconstrained::both()
        } else {
            too::views::Unconstrained::new()
                .horizontal(horizontal)
                .vertical(vertical)
        };

        ui.show_children(view, |ui| ctx.visit_children(mapping, ui));
    }
}
