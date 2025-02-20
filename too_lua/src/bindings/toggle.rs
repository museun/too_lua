use anno_lua::Anno;
use mlua::{AnyUserData, FromLua};
use too::view::Ui;

use crate::{Context, Mapping, None, Spec, View};

#[derive(Clone, Debug, PartialEq, Anno)]
#[anno(exact)]
pub struct ToggleParams {
    /// The boolean state to use
    #[anno(lua_type = "Value")]
    pub value: AnyUserData,
}

impl FromLua for ToggleParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(table) = value else {
            return Err(mlua::Error::runtime(format!(
                "expected ToggleParams, got: {}",
                value.type_name()
            )));
        };

        Ok(Self {
            value: table.get("value")?,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Toggle;

impl View for Toggle {
    type Params = ToggleParams;
    type Style = None;

    fn spec() -> Spec {
        view_spec! {
            /// Conditionally show or hide a view
            Self {
                name: "toggle",
                params: "ToggleParams"
            }
        }
    }

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params::<ToggleParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle", "params");
        };

        let Some(value) = ctx.value_ref(&params.value) else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle", "value");
        };

        let Some(value) = value.bool_ref() else {
            return Mapping::report_missing_data(ui, ctx.id, "toggle", "bool");
        };

        ui.toggle(*value, |ui| ctx.visit_children(mapping, ui));
    }
}
