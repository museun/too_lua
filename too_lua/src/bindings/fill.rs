use anno_lua::Anno;
use mlua::FromLua;
use too::view::Ui;

use crate::{Context, Mapping, None, Spec, View, helper::get_table};

use super::Color;

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct FillParams {
    /// Use this color to fill the area
    #[anno(lua_type = "string")]
    pub background: Color,

    /// Optional width to allocate
    #[anno(lua_type = "integer?")]
    pub width: Option<u16>,

    /// Optional height to allocate
    #[anno(lua_type = "integer?")]
    pub height: Option<u16>,
}

impl FromLua for FillParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                background: table.get("background")?,
                width: table.get("width")?,
                height: table.get("height")?,
            })
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Fill;

impl View for Fill {
    type Params = FillParams;
    type Style = None;

    fn spec() -> Spec {
        view_spec! {
            /// Fill the childrens area, with an optional size constraint
            Self {
                name: "fill",
                params: "FillParams"
            }
        }
    }

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        use too::views::Fill;
        let Some(params) = ctx.params::<FillParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "fill", "params");
        };

        match (params.width, params.height) {
            (Some(w), Some(h)) => {
                ui.show(Fill::new(params.background, (w as i32, h as i32)));
            }
            _ => {
                ui.show(Fill::fill_with(params.background));
            }
        }
    }
}
