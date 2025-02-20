use anno_lua::Anno;
use mlua::FromLua;
use too::view::Ui;

use crate::{helper::get_table, Context, Mapping, None, Register, View};

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

impl mlua::UserData for ConstraintKind {}

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

impl Register for Constraint {
    const NAME: &'static str = "Constraint";
}

impl anno_lua::Anno for Constraint {
    fn lua_type() -> anno_lua::Type {
        anno_lua::Type::Class(anno_lua::Class {
            docs: &["A constraint"],
            name: "Constraint",
            exact: true,
            fields: &[
                anno_lua::Field {
                    name: "exact_size",
                    ty: "fun(w: integer, h: integer): Constraint",
                    docs: &["The view has an exact size"],
                },
                anno_lua::Field {
                    name: "exact_height",
                    ty: "fun(h: integer): Constraint",
                    docs: &["The view has an exact height"],
                },
                anno_lua::Field {
                    name: "exact_width",
                    ty: "fun(w: integer): Constraint",
                    docs: &["The view has an exact width"],
                },
                anno_lua::Field {
                    name: "max_size",
                    ty: "fun(w: integer, h: integer): Constraint",
                    docs: &["The view has a max size"],
                },
                anno_lua::Field {
                    name: "max_height",
                    ty: "fun(h: integer): Constraint",
                    docs: &["The view has a max height"],
                },
                anno_lua::Field {
                    name: "max_width",
                    ty: "fun(w: integer): Constraint",
                    docs: &["The view has a max width"],
                },
                anno_lua::Field {
                    name: "min_size",
                    ty: "fun(w: integer, h: integer): Constraint",
                    docs: &["The view has a min size"],
                },
                anno_lua::Field {
                    name: "min_width",
                    ty: "fun(w: integer): Constraint",
                    docs: &["The view has a min width"],
                },
                anno_lua::Field {
                    name: "min_height",
                    ty: "fun(h: integer): Constraint",
                    docs: &["The view has a min height"],
                },
            ],
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Anno)]
#[anno(extact)]
pub struct ConstrainedParams {
    /// The constraint to use
    #[anno(lua_type = "Constraint")]
    pub constraint: ConstraintKind,
}

impl FromLua for ConstrainedParams {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        get_table(value, |table| {
            Ok(Self {
                constraint: table.get("constraint")?,
            })
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Constrained;

impl View for Constrained {
    type Params = ConstrainedParams;
    type Style = None;

    fn spec() -> crate::binding::Spec {
        view_spec! {
            /// Specifically constrain a view
            Self {
                name: "constrained",
                params: "ConstrainedParams"
            }
        }
    }

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        use too::views::Constrain;
        use ConstraintKind::*;

        let Some(params) = ctx.params::<ConstrainedParams>() else {
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
