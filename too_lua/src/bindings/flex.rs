use anno_lua::Anno;
use too::view::Ui;

use crate::{Context, Mapping, None, View};

#[derive(Copy, Clone, Debug, PartialEq, Anno, serde::Deserialize)]
#[anno(exact)]
pub struct FlexParams {
    /// Tight constraint (ratio between 0.0 and 1.0)
    #[anno(lua_type = "number?")]
    pub tight: Option<f32>,

    /// Loose constraint (ratio between 0.0 and 1.0)
    #[anno(lua_type = "number?")]
    pub loose: Option<f32>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Flex;

impl View for Flex {
    type Params = FlexParams;
    type Style = None;

    fn spec() -> crate::binding::Spec {
        view_spec! {
            /// Give a flex constraint to its children
            Self {
                name: "flex",
                params: "FlexParams"
            }
        }
    }

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        use too::{layout::Flex, views::Flexible};

        let Some(params) = ctx.params_de::<FlexParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "flex", "params");
        };

        if let Some(flex) = params
            .tight
            .map(Flex::Tight)
            .or_else(|| params.loose.map(Flex::Loose))
        {
            ui.show_children(Flexible::new(flex), |ui| ctx.visit_children(mapping, ui));
            return;
        };

        Mapping::report_missing_data(ui, ctx.id, "flex", "params")
    }
}
