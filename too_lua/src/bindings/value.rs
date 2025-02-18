use mlua::{AnyUserData, IntoLua as _};

use crate::{
    proxy::{LuaFunction, Proxy, ProxyKind},
    LuaType,
};

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Float(f32),
    Signed(isize),
    Unsigned(usize),
    String(String),
}

impl Value {
    pub fn bool_ref(&self) -> Option<&bool> {
        match self {
            Self::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn bool_mut(&mut self) -> Option<&mut bool> {
        match self {
            Self::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn float_ref(&self) -> Option<&f32> {
        match self {
            Self::Float(v) => Some(v),
            _ => None,
        }
    }

    pub fn float_mut(&mut self) -> Option<&mut f32> {
        match self {
            Self::Float(v) => Some(v),
            _ => None,
        }
    }

    pub fn signed_ref(&self) -> Option<&isize> {
        match self {
            Self::Signed(v) => Some(v),
            _ => None,
        }
    }

    pub fn signed_mut(&mut self) -> Option<&mut isize> {
        match self {
            Self::Signed(v) => Some(v),
            _ => None,
        }
    }

    pub fn unsigned_ref(&self) -> Option<&usize> {
        match self {
            Self::Unsigned(v) => Some(v),
            _ => None,
        }
    }

    pub fn unsigned_mut(&mut self) -> Option<&mut usize> {
        match self {
            Self::Unsigned(v) => Some(v),
            _ => None,
        }
    }

    pub fn string_ref(&self) -> Option<&String> {
        match self {
            Self::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn string_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::String(v) => Some(v),
            _ => None,
        }
    }
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

impl LuaType for Value {
    const NAME: &'static str = "Value";
    const KIND: ProxyKind = ProxyKind::Value;

    fn lua_functions() -> &'static [LuaFunction] {
        &[
            LuaFunction {
                name: "new fun(value: integer|number|boolean|string): Value",
                doc: "create a new value",
            },
            LuaFunction {
                name: "persist fun(id: string, value: integer|number|boolean|string): Value",
                doc: "create a new value, persisted and accessible via `id`",
            },
            LuaFunction {
                name: "destroy fun(id: string): boolean",
                doc: "destroys a persisted value `id`, if it exists",
            },
            LuaFunction {
                name: "value integer|number|boolean|string",
                doc: "get the inner value",
            },
        ]
    }
}

impl Proxy for Value {}
