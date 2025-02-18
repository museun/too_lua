use too::view::{Palette, Style, StyleOptions};

#[macro_export]
macro_rules! make_struct {
    (struct $name:ident is $lua_name:tt {
        $(
            #[doc = $field_doc:expr]
            $ident:ident = $ty:ty ; $lua_ty:expr
        )*
    }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $(
                #[doc = $field_doc]
                pub $ident: $ty
            ),*
        }

        impl mlua::FromLua for $name {
            fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
                let mlua::Value::Table(value) = value else {
                    return Err(mlua::Error::runtime(
                        format!("expected a table, got {}", value.type_name())
                    ));
                };

                Ok(Self {
                    $( $ident: value.get(stringify!($ident))? ),*
                })
            }
        }

        impl $crate::LuaType for $name {
            const NAME: &'static str = $lua_name;
            const KIND: $crate::ProxyKind = $crate::ProxyKind::Args;

            fn lua_fields() -> &'static [$crate::LuaField] {
                &[
                    $($crate::LuaField {
                        name: stringify!($ident),
                        ty: $lua_ty,
                        doc: $field_doc
                    }),*
                ]
            }
        }
    };
}

#[macro_export]
macro_rules! make_enum {
    (enum $kind:ident is $name:tt {
        $(
            #[doc = $doc:expr]
            $ident:ident = $display:expr
        )*
    }) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[non_exhaustive]
        pub enum $kind {
            $(
                #[doc = $doc]
                $ident
            ),*
        }

        impl $kind {
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$ident => $display),*
                }
            }
        }

        impl mlua::FromLua for $kind {
            fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
                let mlua::Value::UserData(ud) = value else {
                    return Err(mlua::Error::runtime(format!(
                        "expected an {} enum, got a: {}",
                        $name,
                        value.type_name()
                    )));
                };
                ud.borrow::<Self>().map(|c| *c)
            }
        }

        impl mlua::UserData for $kind {
            fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
                for kind in [ $(Self::$ident),* ] {
                    fields.add_field_function_get(kind.as_str(), move |_lua, _| Ok(kind));
                }
            }

            fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
                methods.add_meta_method("__eq", |_lua, this, other: Self| Ok(this == &other));
                methods.add_meta_function("__tostring", |lua, this: Self| {
                    lua.create_string(format!("{}.{}",
                        $name,
                        this.as_str()
                    ))
                })
            }
        }

        impl $crate::LuaType for $kind {
            const NAME: &'static str = $name;
            const KIND: $crate::ProxyKind = $crate::ProxyKind::Enum;

            fn lua_functions() -> &'static [$crate::LuaFunction] {
                &[
                    $($crate::LuaFunction {
                        name: $display,
                        doc: $doc
                    }),*
                ]
            }
        }

        impl $crate::Proxy for $kind {}
    };
}

#[macro_export]
macro_rules! make_class {
    (class $kind:ident is $name:expr ; $proxy:path {
        $(
            #[doc = $doc:expr]
            $ident:ident = $display:expr ; $path:path
        )*
    }) => {
        make_enum! {
            enum $kind is $name {
                $(
                    #[doc = $doc]
                    $ident = $display
                )*
            }
        }

        impl $crate::proxy::TranslateClass for $kind {
            type Style = $proxy;
            fn translate(
                &self,
                palette: &too::view::Palette,
                options: too::view::StyleOptions<<Self::Style as too::view::Style>::Args>,
            ) -> Self::Style {
                match self {
                    $( Self::$ident => $path(palette, options),)*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! make_style {
    (style $kind:ident is $name:expr ; $proxy:path {
        $(
            #[doc = $doc:expr]
            $ident:ident = $display:path ; $lua_ty:expr
        )*
    }) => {
        make_struct! {
            struct $kind is $name {
                $(
                    #[doc = $doc]
                    $ident = $display ; $lua_ty
                )*
            }
        }

        impl $crate::proxy::MergeStyle for $kind {
            type Style = $proxy;
            fn merge_style(&self, style: &mut Self::Style) {
                $( $crate::merge(&mut style.$ident, &self.$ident); )*
            }
        }
    };

    (manual style $kind:ident is $name:expr ; $proxy:path {
        $(
            #[doc = $doc:expr]
            $ident:ident = $display:path ; $lua_ty:expr
        )*
    }) => {
        make_struct! {
            struct $kind is $name {
                $(
                    #[doc = $doc]
                    $ident = $display ; $lua_ty
                )*
            }
        }
    };
}

pub trait Params<S: Style>: 'static + Sized {
    type Class: TranslateClass<Style = S>;
    type Style: MergeStyle<Style = S>;

    fn class(&self) -> &Option<Self::Class>;
    fn style(&self) -> &Option<Self::Style>;

    fn apply_styling(self) -> impl Fn(&Palette, StyleOptions<S::Args>) -> S + 'static {
        move |palette, options| {
            let mut default = match self.class() {
                Some(class) => class.translate(palette, options),
                None => too::view::Style::default(palette, options),
            };
            if let Some(style) = self.style() {
                style.merge_style(&mut default);
            }
            default
        }
    }
}

pub trait TranslateClass: 'static {
    type Style: Style;
    fn translate(
        &self,
        palette: &Palette,
        options: StyleOptions<<Self::Style as Style>::Args>,
    ) -> Self::Style;
}

pub trait MergeStyle: 'static {
    type Style;
    fn merge_style(&self, style: &mut Self::Style);
}
