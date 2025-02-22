use anno_lua::Anno;
use mlua::UserData;
use too::view::{Palette, Style, StyleOptions, Ui};

use crate::{Context, Mapping};

#[macro_export]
macro_rules! view_spec {
    (
        $(#[doc = $doc:expr_2021])*
        $ty:ty {
            name: $name:expr_2021
        }
    ) => {
        $crate::binding::Spec {
            docs: &[ $($doc),* ],
            name: $name,
            args: $crate::binding::Arguments::None,
            params: <<$ty as $crate::binding::View>::Params as anno_lua::Anno>::lua_type,
            style: <<$ty as $crate::binding::View>::Style as anno_lua::Anno>::lua_type,
            associated: &[],
            proxies: &[]
        }
    };

    (
        $(#[doc = $doc:expr_2021])*
        $ty:ty {
            name: $name:expr_2021,
            params: any
        }
    ) => {
        $crate::binding::Spec {
            docs: &[ $($doc),* ],
            name: $name,
            args: $crate::binding::Arguments::Any,
            params: <<$ty as $crate::binding::View>::Params as anno_lua::Anno>::lua_type,
            style: <<$ty as $crate::binding::View>::Style as anno_lua::Anno>::lua_type,
            associated: &[],
            proxies: &[]
        }
    };

    (
        $(#[doc = $doc:expr_2021])*
        $ty:ty {
            name: $name:expr_2021,
            params: $($params:tt $(|$tail:tt)*)?
        }
    ) => {
        $crate::binding::Spec {
            docs: &[ $($doc),* ],
            name: $name,
            args: $crate::binding::Arguments::Named(
                concat!($($params, $("|", $tail,)*)?)
            ),
            params: <<$ty as $crate::binding::View>::Params as anno_lua::Anno>::lua_type,
            style: <<$ty as $crate::binding::View>::Style as anno_lua::Anno>::lua_type,
            associated: &[],
            proxies: &[]
        }
    };
}

#[macro_export]
macro_rules! register_enum {
    ($ident:ident is $name:expr_2021) => {
        impl $crate::binding::Register for $ident {
            const NAME: &'static str = $name;
        }

        impl mlua::FromLua for $ident {
            fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
                let mlua::Value::UserData(ud) = value else {
                    return Err(mlua::Error::runtime(format!(
                        "expected {} ({}), got: {}",
                        $name,
                        stringify!($ident),
                        value.type_name(),
                    )));
                };
                ud.borrow::<Self>().map(|c| *c)
            }
        }

        impl mlua::UserData for $ident {
            fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
                use anno_lua::AnnoEnum as _;
                for (k, v) in Self::variants() {
                    fields.add_field_function_get(k, |_lua, _this| Ok(*v));
                }
            }
        }
    };
}

pub trait Register: UserData + 'static {
    const NAME: &'static str;

    fn set_proxy(table: &mlua::Table, lua: &mlua::Lua) -> mlua::Result<()> {
        table.set(Self::NAME, lua.create_proxy::<Self>()?)
    }

    fn proxy() -> Proxy
    where
        Self: Anno,
    {
        const { Proxy::new::<Self>() }
    }
}

pub trait RegisterProxy {
    fn proxy<T>(&self) -> mlua::Result<()>
    where
        T: Register + Anno,
    {
        self.add_proxy(const { Proxy::new::<T>() })
    }

    fn add_proxy(&self, proxy: Proxy) -> mlua::Result<()>;
}

impl RegisterProxy for mlua::Lua {
    fn add_proxy(&self, proxy: Proxy) -> mlua::Result<()> {
        (proxy.register)(&self.globals(), self)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Proxy {
    pub ty: fn() -> anno_lua::Type,
    pub register: fn(&mlua::Table, &mlua::Lua) -> mlua::Result<()>,
}

impl Proxy {
    pub const fn new<T>() -> Self
    where
        T: Register + Anno,
    {
        Self {
            ty: T::lua_type,
            register: T::set_proxy,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Spec {
    pub docs: &'static [&'static str],
    pub name: &'static str,
    pub args: Arguments,
    pub style: fn() -> anno_lua::Type,
    pub params: fn() -> anno_lua::Type,
    pub proxies: &'static [Proxy],
    pub associated: &'static [fn() -> anno_lua::Type],
}

impl Spec {
    pub fn params(&self) -> Option<anno_lua::Type> {
        let ty = (self.params)();
        Self::is_not_unit(&ty).then_some(ty)
    }

    pub fn style(&self) -> Option<anno_lua::Type> {
        let ty = (self.style)();
        Self::is_not_unit(&ty).then_some(ty)
    }

    pub const fn with_proxies(mut self, proxies: &'static [Proxy]) -> Self {
        self.proxies = proxies;
        self
    }

    pub const fn with_associated(mut self, associated: &'static [fn() -> anno_lua::Type]) -> Self {
        self.associated = associated;
        self
    }

    fn is_not_unit(ty: &anno_lua::Type) -> bool {
        !matches!(
            ty,
            anno_lua::Type::Class(anno_lua::Class { name: "()", .. })
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Arguments {
    Any,
    None,
    Named(&'static str),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct None;

impl Anno for None {
    fn lua_type() -> anno_lua::Type {
        anno_lua::Type::Class(anno_lua::Class {
            exact: false,
            docs: &[],
            name: "()",
            fields: &[],
        })
    }
}

pub trait View {
    type Params: Anno;
    type Style: Anno;

    fn spec() -> Spec;
    fn view(mapping: &Mapping, ui: &Ui, ctx: Context);
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
                Option::None => too::view::Style::default(palette, options),
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
