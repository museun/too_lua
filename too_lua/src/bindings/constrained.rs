use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    proxy::{LuaFunction, Proxy, ProxyKind},
    Context, LuaType, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ConstraintKind {
    ExactSize(u16, u16),
    MaxSize(u16, u16),
    MinSize(u16, u16),
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

pub struct Constraint;

impl mlua::UserData for Constraint {
    fn add_methods<M>(methods: &mut M)
    where
        M: mlua::UserDataMethods<Self>,
    {
        use ConstraintKind::*;
        for (name, ctor) in &[
            ("exact_size", ExactSize as fn(u16, u16) -> _),
            ("max_size", MaxSize),
            ("min_size", MinSize),
        ] {
            methods.add_function(name, move |_lua, (w, h)| Ok(ctor(w, h)));
        }

        for (name, ctor) in &[
            ("exact_height", ExactHeight as fn(u16) -> _),
            ("exact_width", ExactWidth),
            ("max_height", MaxHeight),
            ("max_width", MaxWidth),
            ("min_width", MinWidth),
            ("min_height", MinHeight),
        ] {
            methods.add_function(name, move |_lua, e| Ok(ctor(e)));
        }
    }
}

impl LuaType for Constraint {
    const NAME: &'static str = "Constraint";
    const KIND: ProxyKind = ProxyKind::Value;

    fn lua_functions() -> &'static [LuaFunction] {
        &[
            LuaFunction {
                name: "exact_size fun(w: integer, h: integer): Constraint",
                doc: "The view has an exact size",
            },
            LuaFunction {
                name: "exact_height fun(h: integer): Constraint",
                doc: "The view has an exact height",
            },
            LuaFunction {
                name: "exact_width fun(w: integer): Constraint",
                doc: "The view has an exact width",
            },
            LuaFunction {
                name: "max_size fun(w: integer, h: integer): Constraint",
                doc: "The view has a max size",
            },
            LuaFunction {
                name: "max_height fun(h: integer): Constraint",
                doc: "The view has a max height",
            },
            LuaFunction {
                name: "max_width fun(w: integer): Constraint",
                doc: "The view has a max width",
            },
            LuaFunction {
                name: "min_size fun(w: integer, h: integer): Constraint",
                doc: "The view has a min size",
            },
            LuaFunction {
                name: "min_width fun(w: integer): Constraint",
                doc: "The view has a min width",
            },
            LuaFunction {
                name: "min_height fun(h: integer): Constraint",
                doc: "The view has a min height",
            },
        ]
    }
}

impl Proxy for Constraint {}

crate::make_struct! {
    struct ConstrainedParams is "ConstrainedParams" {
        /// The constraint to use
        constraint = ConstraintKind ; "Constraint"
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Constrained;

impl BindingView for Constrained {
    const SPEC: BindingSpec = binding! {
        /// Specifically constrain a view
        "constrained" => "ConstrainedParams"
    };

    type Params = ConstrainedParams;
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        use too::views::Constrain;
        use ConstraintKind::*;

        let Some(params) = ctx.foo::<ConstrainedParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "constrained", "params");
        };

        let view = match params.constraint {
            ExactSize(w, h) => Constrain::exact_size((w as i32, h as i32)),
            MaxSize(w, h) => Constrain::max_size((w as i32, h as i32)),
            MinSize(w, h) => Constrain::min_size((w as i32, h as i32)),
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
