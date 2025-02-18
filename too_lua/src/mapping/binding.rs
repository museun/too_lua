use too::view::Ui;

use super::{Context, Mapping};
use crate::{LuaField, LuaFunction, LuaType};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Binding {
    pub name: &'static str,
    pub doc: &'static str,
    pub fields: fn() -> &'static [LuaField],
    pub bindings: fn() -> &'static [LuaFunction],
    pub args: BindingArgs,
    pub params: BindingParams,
}

impl Binding {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            doc: "",
            fields: || &[],
            bindings: || &[],
            args: BindingArgs::None,
            params: BindingParams {
                name: "",
                fields: &[],
            },
        }
    }

    pub const fn from_ty<T: LuaType>(name: &'static str) -> Self {
        Self {
            fields: T::lua_fields,
            bindings: T::lua_functions,
            ..Self::new(name)
        }
    }

    pub const fn doc(mut self, doc: &'static str) -> Self {
        self.doc = doc;
        self
    }

    pub const fn args(mut self, args: BindingArgs) -> Self {
        self.args = args;
        self
    }

    pub const fn params(mut self, params: BindingParams) -> Self {
        self.params = params;
        self
    }

    pub fn style<T: LuaType>(mut self) -> Self {
        self.params = BindingParams {
            name: T::NAME,
            fields: T::lua_fields(),
        };
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BindingSpec {
    pub name: &'static str,
    pub docs: &'static str,
    pub params: BindingArgs,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BindingArgs {
    Any,
    Named(&'static str),
    None,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BindingParams {
    pub name: &'static str,
    pub fields: &'static [LuaField],
}

// $(=> $head:tt $(| $tail:tt)*)?
// Some(concat!($($head, $("|", $tail,)*)?)),
#[macro_export]
macro_rules! binding {
    (
        #[doc = $doc:expr]
        $name:expr => any
    ) => {
        $crate::mapping::BindingSpec {
            name: $name,
            docs: $doc,
            params: $crate::mapping::BindingArgs::Any,
        }
    };

    (
        #[doc = $doc:expr]
        $name:expr => $params:expr
    ) => {
        $crate::mapping::BindingSpec {
            name: $name,
            docs: $doc,
            params: $crate::mapping::BindingArgs::Named($params),
        }
    };

    (
        #[doc = $doc:expr]
        $name:expr
    ) => {
        $crate::mapping::BindingSpec {
            name: $name,
            docs: $doc,
            params: $crate::mapping::BindingArgs::None,
        }
    };
}

pub trait BindingView {
    const SPEC: BindingSpec;
    type Params: LuaType;
    type Style: LuaType;

    fn binding() -> Binding {
        Binding::from_ty::<Self::Params>(Self::SPEC.name)
            .doc(Self::SPEC.docs)
            .args(Self::SPEC.params)
            .style::<Self::Style>()
    }

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context);
}
