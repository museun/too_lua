#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Constraint {
    ExactSize { w: u16, h: u16 },
    MaxSize { w: u16, h: u16 },
    MinSize { w: u16, h: u16 },
    //
    ExactHeight(u16),
    ExactWidth(u16),
    MaxHeight(u16),
    MaxWidth(u16),
    MinWidth(u16),
    MinHeight(u16),
}

impl mlua::FromLua for Constraint {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::UserData(ud) = value else {
            return Err(mlua::Error::runtime(format!(
                "expected Constraint, got {}",
                value.type_name()
            )));
        };
        ud.take()
    }
}

impl mlua::UserData for Constraint {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method("__tostring", |lua, this: &Self, ()| {
            lua.create_string(format!("{this:?}"))
        });

        methods.add_meta_method("__eq", |_lua, this: &Self, other: Self| {
            Ok(this == &other) //
        });
    }
}

pub struct Constrained;

impl mlua::UserData for Constrained {
    fn add_methods<M>(methods: &mut M)
    where
        M: mlua::UserDataMethods<Self>,
    {
        methods.add_function("exact_size", |_lua, (w, h): (u16, u16)| {
            let constraint = Constraint::ExactSize { w, h };
            Ok(constraint)
        });

        methods.add_function("exact_height", |_lua, h: u16| {
            let constraint = Constraint::ExactHeight(h);
            Ok(constraint)
        });

        methods.add_function("exact_width", |_lua, w: u16| {
            let constraint = Constraint::ExactWidth(w);
            Ok(constraint)
        });

        methods.add_function("max_size", |_lua, (w, h): (u16, u16)| {
            let constraint = Constraint::MaxSize { w, h };
            Ok(constraint)
        });

        methods.add_function("max_height", |_lua, h: u16| {
            let constraint = Constraint::MaxHeight(h);
            Ok(constraint)
        });

        methods.add_function("max_width", |_lua, w: u16| {
            let constraint = Constraint::MaxWidth(w);
            Ok(constraint)
        });

        methods.add_function("min_size", |_lua, (w, h): (u16, u16)| {
            let constraint = Constraint::MinSize { w, h };
            Ok(constraint)
        });

        methods.add_function("min_width", |_lua, w: u16| {
            let constraint = Constraint::MinWidth(w);
            Ok(constraint)
        });

        methods.add_function("min_height", |_lua, h: u16| {
            let constraint = Constraint::MinHeight(h);
            Ok(constraint)
        });
    }
}

impl crate::params::Proxy for Constrained {
    fn create(lua: &mlua::Lua) -> mlua::Result<()> {
        lua.globals()
            .set("Constrained", lua.create_proxy::<Self>()?)
    }
}
