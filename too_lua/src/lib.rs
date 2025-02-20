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
pub use extract::{Extract, merge};

mod generate;
pub use generate::generate;

mod helper;
