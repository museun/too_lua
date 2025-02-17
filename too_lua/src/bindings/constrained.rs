use too::view::Ui;

use crate::{
    mapping::{Binding, Field},
    proxy::{Proxy, ProxyKind},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ConstraintKind {
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

impl mlua::FromLua for ConstraintKind {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::UserData(ud) = value else {
            return Err(mlua::Error::runtime(format!(
                "expected Constraint, got {}",
                value.type_name()
            )));
        };
        ud.borrow::<Self>().map(|c| *c)
    }
}

impl mlua::UserData for ConstraintKind {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method("__tostring", |lua, this: &Self, ()| {
            lua.create_string(format!("{this:?}"))
        });

        methods.add_meta_method("__eq", |_lua, this: &Self, other: Self| {
            Ok(this == &other) //
        });
    }
}

// why is there a third one?
pub struct Constraint;

impl Constraint {
    const TUPLE_CONSTRUCTORS: &[(&str, fn(u16, u16) -> ConstraintKind)] = &[
        ("exact_size", |w, h| ConstraintKind::ExactSize { w, h }),
        ("max_size", |w, h| ConstraintKind::MaxSize { w, h }),
        ("min_size", |w, h| ConstraintKind::MinSize { w, h }),
    ];

    const SINGLE_CONSTRUCTORS: &[(&str, fn(u16) -> ConstraintKind)] = &[
        ("exact_height", |h| ConstraintKind::ExactHeight(h)),
        ("exact_width", |w| ConstraintKind::ExactWidth(w)),
        ("max_height", |h| ConstraintKind::MaxHeight(h)),
        ("max_width", |w| ConstraintKind::MaxWidth(w)),
        ("min_width", |w| ConstraintKind::MinWidth(w)),
        ("min_height", |h| ConstraintKind::MinHeight(h)),
    ];
}

impl mlua::UserData for Constraint {
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

impl Proxy for Constraint {
    const KIND: ProxyKind = ProxyKind::Value;
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Constrained;

impl Constrained {
    binding! {
        /// Specifically constrain a view
        "constrained" => "constrained" {
            /// The constraint to use
            constraint "Constraint"
        }
    }

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        use too::views::Constrain;
        use ConstraintKind::*;

        let Some(Ok(constraint)) = ctx.params_field::<ConstraintKind>("constraint") else {
            return Mapping::report_missing_data(ui, ctx.id, "constrained", "constraint");
        };

        let view = match constraint {
            ExactSize { w, h } => Constrain::exact_size((w as i32, h as i32)),
            MaxSize { w, h } => Constrain::max_size((w as i32, h as i32)),
            MinSize { w, h } => Constrain::min_size((w as i32, h as i32)),
            ExactHeight(v) => Constrain::exact_height(v as i32),
            ExactWidth(v) => Constrain::exact_width(v as i32),
            MaxHeight(v) => Constrain::max_height(v as i32),
            MaxWidth(v) => Constrain::max_width(v as i32),
            MinWidth(v) => Constrain::min_width(v as i32),
            MinHeight(v) => Constrain::min_height(v as i32),
        };

        ui.show_children(view, |ui| ctx.visit_children(mapping, ui));
    }
}
