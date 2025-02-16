#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Constraint {
    ExactSize { w: u16, h: u16 },
    MaxSize { w: u16, h: u16 },
    MinSize { w: u16, h: u16 },
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

impl Constrained {
    const TUPLE_CONSTRUCTORS: &[(&str, fn(u16, u16) -> Constraint)] = &[
        ("exact_size", |w, h| Constraint::ExactSize { w, h }),
        ("max_size", |w, h| Constraint::MaxSize { w, h }),
        ("min_size", |w, h| Constraint::MinSize { w, h }),
    ];

    const SINGLE_CONSTRUCTORS: &[(&str, fn(u16) -> Constraint)] = &[
        ("exact_height", |h| Constraint::ExactHeight(h)),
        ("exact_width", |w| Constraint::ExactWidth(w)),
        ("max_height", |h| Constraint::MaxHeight(h)),
        ("max_width", |w| Constraint::MaxWidth(w)),
        ("min_width", |w| Constraint::MinWidth(w)),
        ("min_height", |h| Constraint::MinHeight(h)),
    ];
}

impl mlua::UserData for Constrained {
    fn add_methods<M>(methods: &mut M)
    where
        M: mlua::UserDataMethods<Self>,
    {
        for (name, ctor) in Self::TUPLE_CONSTRUCTORS {
            methods.add_function(name, move |_lua, (w, h)| Ok(ctor(w, h)));
        }

        for (name, ctor) in Self::SINGLE_CONSTRUCTORS {
            methods.add_function(name, move |_lua, e| Ok(ctor(e)));
        }
    }
}

impl crate::params::Proxy for Constrained {
    const KIND: super::Kind = super::Kind::Value;
    const NAME: &'static str = "Constraint";
    const STYLE: Option<fn() -> &'static [(&'static str, &'static str, &'static str)]> = None;

    fn lua_bindings() -> &'static [(&'static str, &'static str)] {
        &[
            (
                "exact_size fun(w: integer, h: integer): Constraint",
                "The view has an exact size",
            ),
            (
                "exact_height fun(h: integer): Constraint",
                "The view has an exact height",
            ),
            (
                "exact_width fun(w: integer): Constraint",
                "The view has an exact width",
            ),
            (
                "max_size fun(w: integer, h: integer): Constraint",
                "The view has a max size",
            ),
            (
                "max_height fun(h: integer): Constraint",
                "The view has a max height",
            ),
            (
                "max_width fun(w: integer): Constraint",
                "The view has a max width",
            ),
            (
                "min_size fun(w: integer, h: integer): Constraint",
                "The view has a min size",
            ),
            (
                "min_width fun(w: integer): Constraint",
                "The view has a min width",
            ),
            (
                "min_height fun(h: integer): Constraint",
                "The view has a min height",
            ),
        ]
    }
}
