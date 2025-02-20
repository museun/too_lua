use anno_lua::Anno;
use mlua::FromLua;
use too::view::Ui;

use crate::{Context, Mapping, None, Spec, View, helper::get_table};

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
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

impl FromLua for UnconstrainedParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                horizontal: table.get("horizontal")?,
                vertical: table.get("vertical")?,
                both: table.get("both")?,
            })
        })
    }
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
        let Some(params) = ctx.params::<UnconstrainedParams>() else {
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
