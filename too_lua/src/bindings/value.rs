use mlua::{AnyUserData, IntoLua as _};

use crate::proxy::{Proxy, ProxyKind};

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Float(f32),
    Signed(isize),
    Unsigned(usize),
    String(String),
}

impl Value {
    const GLOBAL_KEY: &'static str = "__TOO_VALUES";
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

        methods.add_function(
            "persist",
            |lua, (id, value): (mlua::String, mlua::Value)| {
                let table = match lua.globals().get::<mlua::Table>(Self::GLOBAL_KEY) {
                    Ok(table) => table,
                    Err(..) => {
                        let table = lua.create_table()?;
                        lua.globals().set(Self::GLOBAL_KEY, table.clone())?;
                        table
                    }
                };

                match table.get::<AnyUserData>(&id) {
                    Ok(value) => value.into_lua(lua),
                    Err(..) => {
                        let this = match value {
                            mlua::Value::Boolean(value) => Self::Bool(value),
                            mlua::Value::Integer(value) => Self::Signed(value as isize),
                            mlua::Value::Number(value) => Self::Float(value as f32),
                            mlua::Value::String(value) => Self::String(value.to_string_lossy()),
                            _ => return Err(mlua::Error::runtime("invalid type")),
                        };
                        let this = this.into_lua(lua)?;
                        table.set(id, this.clone())?;
                        Ok(this)
                    }
                }
            },
        );

        methods.add_function("destroy", |lua, id: mlua::String| {
            if let Ok(table) = lua.globals().get::<mlua::Table>(Self::GLOBAL_KEY) {
                return Ok(table.set(id, mlua::Value::Nil).is_ok());
            }
            Ok(false)
        });
    }
}

impl Proxy for Value {
    const KIND: ProxyKind = ProxyKind::Value;
    const NAME: &'static str = "Value";
    const STYLE: Option<fn() -> &'static [(&'static str, &'static str, &'static str)]> = None;

    fn lua_bindings() -> &'static [(&'static str, &'static str)] {
        &[
            (
                "new fun(value: integer|number|boolean|string): Value",
                "create a new value",
            ),
            (
                "persist fun(id: string, value: integer|number|boolean|string): Value",
                "create a new value, persisted and accessible via `id`",
            ),
            (
                "destroy fun(id: string): boolean",
                "destroys a persisted value `id`, if it exists",
            ),
            (
                "value fun(value: Value): integer|number|boolean|string",
                "get the inner value",
            ),
            (
                "value integer|number|boolean|string", //
                "set the inner value",
            ),
        ]
    }
}
