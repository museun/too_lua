use anno_lua::Anno;
use mlua::FromLua;
use too::view::Ui;

use crate::{helper::get_table, Context, Mapping, None, Spec, View};

use super::Color;

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
pub struct BackgroundParams {
    /// The background color for the children
    #[anno(lua_type = "string")]
    pub background: Color,
}

impl FromLua for BackgroundParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                background: table.get("background")?,
            })
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Background;

impl View for Background {
    type Params = BackgroundParams;
    type Style = None;

    fn spec() -> Spec {
        view_spec! {
            /// Applies a background color to this children
            Self {
                name: "background",
                params: "BackgroundParams"
            }
        }
    }

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params::<BackgroundParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "background", "params");
        };

        ui.background(params.background, |ui| ctx.visit_children(mapping, ui));
    }
}
