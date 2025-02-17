use mlua::IntoLua as _;

use crate::proxy::{ProxyKind, Proxy};

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Float(f32),
    Signed(isize),
    Unsigned(usize),
    String(String),
}

impl mlua::UserData for Value {
    fn add_fields<F>(fields: &mut F)
    where
        F: mlua::UserDataFields<Self>,
    {
        fields.add_field_method_get("value", |lua, this| match this {
            Self::Bool(value) => value.into_lua(lua),
            Self::Float(value) => value.into_lua(lua),
            Self::Signed(value) => value.into_lua(lua),
            Self::Unsigned(value) => value.into_lua(lua),
            Self::String(value) => value.as_str().into_lua(lua),
        });

        fields.add_field_method_set("value", |_lua, this, value: mlua::Value| {
            *this = match value {
                mlua::Value::Boolean(value) => Self::Bool(value),
                mlua::Value::Integer(value) => Self::Signed(value as isize),
                mlua::Value::Number(value) => Self::Float(value as f32),
                mlua::Value::String(value) => Self::String(value.to_string_lossy()),
                _ => return Err(mlua::Error::runtime("invalid type")),
            };
            Ok(())
        });
    }

    fn add_methods<M>(methods: &mut M)
    where
        M: mlua::UserDataMethods<Self>,
    {
        methods.add_function("new", |lua, value: mlua::Value| {
            let this = match value {
                mlua::Value::Boolean(value) => Self::Bool(value),
                mlua::Value::Integer(value) => Self::Signed(value as isize),
                mlua::Value::Number(value) => Self::Float(value as f32),
                mlua::Value::String(value) => Self::String(value.to_string_lossy()),
                _ => return Err(mlua::Error::runtime("invalid type")),
            };
            this.into_lua(lua)
        });
    }
}

impl Proxy for Value {
    const KIND: ProxyKind = ProxyKind::Value;
    const NAME: &'static str = "Value";
    const STYLE: Option<fn() -> &'static [(&'static str, &'static str, &'static str)]> = None;

    fn lua_bindings() -> &'static [(&'static str, &'static str)] {
        &[("new fun(value: any): Value", "create a new value")]
    }
}
