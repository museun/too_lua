use too::renderer::Rgba;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(pub Rgba);

impl mlua::FromLua for Color {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let Some(str) = value.as_str() else {
            return Err(mlua::Error::runtime(format!(
                "expected a string, got a {}",
                value.type_name()
            )));
        };

        str.parse::<Rgba>().map(Self).map_err(mlua::Error::runtime)
    }
}

impl From<Color> for Rgba {
    fn from(value: Color) -> Self {
        value.0
    }
}
