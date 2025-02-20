#[macro_use]
mod binding;
pub use binding::{
    Arguments, MergeStyle, None, Params, Proxy, Register, Spec, TranslateClass, View,
};

mod application;
pub use application::Application;

mod script;
pub use script::Script;

mod runtime;

#[macro_use]
mod mapping;
pub use mapping::{Context, Indirect, Mapping};

mod tree;
pub use tree::{DebugNode, Tree};
use tree::{LuaId, Node, UiBuilder};

mod errors;
use errors::Errors;

mod notifications;
use notifications::{Notification, Notifications};

pub mod bindings;
#[doc(inline)]
pub use bindings::Bindings;

mod extract;
pub use extract::{merge, Extract};

mod generate;
pub use generate::generate;

mod helper {
    // TODO short name
    pub fn get_table<T>(
        value: mlua::Value,
        extract: impl FnOnce(mlua::Table) -> mlua::Result<T>,
    ) -> mlua::Result<T> {
        let mlua::Value::Table(table) = value else {
            return Err(mlua::Error::runtime(format!(
                "expected {}, got: {}",
                std::any::type_name::<T>(),
                value.type_name()
            )));
        };
        extract(table)
    }
}
