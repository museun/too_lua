use too::renderer::Rgba;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(pub Rgba);

impl mlua::FromLua for Color {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::String(value) = value else {
            return Err(mlua::Error::runtime(format!(
                "expected a string, got: {}",
                value.type_name(),
            )));
        };

        let color = value
            .to_str()?
            .parse::<Rgba>()
            .map_err(mlua::Error::external)?;
        Ok(Self(color))
    }
}
