use anno_lua::Anno;
use too::view::Ui;

use crate::{Context, Mapping, None, Spec, View};

use super::Color;

#[derive(Copy, Clone, Debug, PartialEq, Anno, serde::Deserialize)]
pub struct BackgroundParams {
    /// The background color for the children
    #[anno(lua_type = "string")]
    pub background: Color,
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
        let Some(params) = ctx.params_de::<BackgroundParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "background", "params");
        };

        ui.background(params.background, |ui| ctx.visit_children(mapping, ui));
    }
}

#[cfg(test)]
mod tests {
    use mlua::LuaSerdeExt;

    use super::*;
    #[test]
    fn asdf() {
        let lua = mlua::Lua::new();

        let v = lua
            .load(
                r##"
        return {
            background = "#e230aadd"
        }
        "##,
            )
            .eval::<mlua::Value>()
            .unwrap();
        let params: BackgroundParams = lua.from_value(v).unwrap();
        eprintln!("{params:#?}")
    }
}
