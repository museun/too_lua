use mlua::LuaSerdeExt as _;
use too::renderer::Rgba;

#[derive(Copy, Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Color(
    #[serde(deserialize_with = "crate::serde::from_str")] //
    pub  Rgba,
);

impl mlua::FromLua for Color {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

impl From<Color> for Rgba {
    fn from(value: Color) -> Self {
        value.0
    }
}
