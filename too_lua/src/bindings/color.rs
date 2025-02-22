use anno_lua::Anno;
use mlua::UserData;
use too::renderer::Rgba;

use crate::{Register, builtin::PaletteKind};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(pub Rgba);

impl UserData for Color {}

impl mlua::FromLua for Color {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::String(str) => str
                .to_str()?
                .parse::<Rgba>()
                .map(Self)
                .map_err(mlua::Error::runtime),

            mlua::Value::UserData(ud) => ud.borrow::<Self>().map(|c| *c).or_else(|_| {
                let kind = ud.borrow::<PaletteKind>()?;
                let p = lua.app_data_ref::<too::view::Palette>().unwrap();
                let rgba = match &*kind {
                    PaletteKind::Background => p.background,
                    PaletteKind::Foreground => p.foreground,
                    PaletteKind::Surface => p.surface,
                    PaletteKind::Outline => p.outline,
                    PaletteKind::Contrast => p.contrast,
                    PaletteKind::Primary => p.primary,
                    PaletteKind::Secondary => p.secondary,
                    PaletteKind::Accent => p.accent,
                    PaletteKind::Danger => p.danger,
                    PaletteKind::Success => p.success,
                    PaletteKind::Warning => p.warning,
                    PaletteKind::Info => p.info,
                };

                Ok(Self(rgba))
            }),

            _ => Err(mlua::Error::runtime(format!(
                "expected a string|Color, got a {}",
                value.type_name()
            ))),
        }
    }
}

impl Anno for Color {
    fn lua_type() -> anno_lua::Type {
        anno_lua::Type::Class(anno_lua::Class {
            exact: true,
            docs: &["#RGB | #RGBA | #RRGGBB | #RRGGBBAA hex string"],
            name: "Color",
            fields: &[],
        })
    }
}

impl From<Color> for Rgba {
    fn from(value: Color) -> Self {
        value.0
    }
}

impl Register for Color {
    const NAME: &'static str = "Color";
}
