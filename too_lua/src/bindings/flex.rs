use anno_lua::Anno;
use mlua::FromLua;
use too::view::Ui;

use crate::{Context, Mapping, None, View, binding::Spec, helper::expect_table};

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct FlexParams {
    /// Tight constraint (ratio between 0.0 and 1.0)
    #[anno(lua_type = "number?")]
    pub tight: Option<f32>,

    /// Loose constraint (ratio between 0.0 and 1.0)
    #[anno(lua_type = "number?")]
    pub loose: Option<f32>,
}

impl FromLua for FlexParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        expect_table(&value, |table| {
            Ok(Self {
                tight: table.get("tight")?,
                loose: table.get("loose")?,
            })
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Flex;

impl View for Flex {
    type Params = FlexParams;
    type Style = None;

    fn spec() -> Spec {
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

        let Some(params) = ctx.params::<FlexParams>() else {
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
