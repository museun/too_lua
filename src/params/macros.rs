#[macro_export]
macro_rules! make_style {
    (@no_merge $name:ident => $style:path { $($ident:ident = $ty:ty)* }) => {
        pub struct $name {
            $(pub $ident: $ty),*
        }

        impl mlua::FromLua for $name {
            fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
                let mlua::Value::Table(value) = value else {
                    return Err(mlua::Error::runtime("expected a table"));
                };

                Ok(Self {
                    $($ident : value.get(stringify!($ident))?),*
                })
            }
        }
    };

    ($name:ident => $style:path { $($ident:ident = $ty:ty)* }) => {
        pub struct $name {
            $(pub $ident: $ty),*
        }

        impl $name {
            pub fn merge_style(&self, style: &mut $style) {
                $(
                    $crate::params::merge(&mut style.$ident, &self.$ident);
                )*
            }
        }

        impl mlua::FromLua for $name {
            fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
                let mlua::Value::Table(value) = value else {
                    return Err(mlua::Error::runtime("expected a table"));
                };

                Ok(Self {
                    $($ident : value.get(stringify!($ident))?),*
                })
            }
       }
    };
}

#[macro_export]
macro_rules! make_class {
    ($kind:ident { $($ident:ident = $display:expr)* }) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[non_exhaustive]
        pub enum $kind {
            $($ident),*
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
                        stringify!($kind),
                        value.type_name()
                    )));
                };
                ud.take()
            }
        }

        impl mlua::UserData for $kind {
            fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
                for kind in [
                    $(Self::$ident),*
                ] {
                    fields.add_field_function_get(kind.as_str(), move |_lua, _| Ok(kind));
                }
            }

            fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
                methods.add_meta_method("__eq", |_lua, this, other: Self| Ok(this == &other));
                methods.add_meta_function("__tostring", |lua, this: Self| {
                    lua.create_string(format!("{}.{}",
                        stringify!($kind),
                        this.as_str()
                    ))
                })
            }
        }

        impl crate::params::Proxy for $kind {
            fn create(lua: &mlua::Lua) -> mlua::Result<()>{
                lua.globals().set(stringify!($kind), lua.create_proxy::<Self>()?)
            }
        }
    };
}

#[macro_export]
macro_rules! make_proxy {
    (
        $params_ident:ident {
            class: $class_ident:ident { $($variant:ident = $display:expr ; $style_func:path)* }
            style: $style_ident:ident => $proxy_name:path { $($field:ident = $ty:ty)* }
        }
    ) => {
        make_class! {
            $class_ident {
                $($variant = $display)*
            }
        }

        make_style! {
            $style_ident => $proxy_name {
                $($field = $ty)*
            }
        }

        make_proxy!(@inner
            $params_ident {
                class: $class_ident { $($variant = $display ; $style_func)* }
                style: $style_ident => $proxy_name { $($field = $ty)* }
            }
        );

        make_proxy!(@apply
            $params_ident {
                class: $class_ident { $($variant = $display ; $style_func)* }
                style: $style_ident => $proxy_name { $($field = $ty)* }
            }
        );
    };

    (
        $params_ident:ident {
            class: $class_ident:ident { $($variant:ident = $display:expr ; $style_func:path)* }
            manual style: $style_ident:ident => $proxy_name:path { $($field:ident = $ty:ty)* }
        }
    ) => {
        make_class! {
            $class_ident {
                $($variant = $display)*
            }
        }

        make_style! {
            @no_merge $style_ident => $proxy_name {
                $($field = $ty)*
            }
        }

        make_proxy!(@inner
            $params_ident {
                class: $class_ident { $($variant = $display ; $style_func)* }
                style: $style_ident => $proxy_name { $($field = $ty)* }
            }
        );
    };

    (@apply
        $params_ident:ident {
            class: $class_ident:ident { $($variant:ident = $display:expr ; $style_func:path)* }
            style: $style_ident:ident => $proxy_name:path { $($field:ident = $ty:ty)* }
    }) => {
        impl $params_ident {
            pub fn apply(self) -> impl Fn(&Palette, StyleOptions<<$proxy_name as Style>::Args>) -> $proxy_name {
                move |palette, options| {
                    let mut default = match self.class {
                        $(
                            Some($class_ident::$variant) => $style_func(palette, options),
                        )*
                        None => Style::default(palette, options)
                    };

                    if let Some(style) = &self.style {
                        style.merge_style(&mut default);
                    }
                    default
                }
            }
        }
    };

    (@inner
        $params_ident:ident {
            class: $class_ident:ident { $($variant:ident = $display:expr ; $style_func:path)* }
            style: $style_ident:ident => $proxy_name:path { $($field:ident = $ty:ty)* }
        }
    ) => {
        pub struct $params_ident {
            pub style: Option<$style_ident>,
            pub class: Option<$class_ident>,
        }

        impl mlua::FromLua for $params_ident {
            fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
                let Some(table) = value.as_table() else {
                    return Err(mlua::Error::runtime(format!("expected a table, got {}", value.type_name())));
                };

                Ok(Self {
                    style: table.get("style")?,
                    class: table.get("class")?,
                })
            }
        }
    };
}
