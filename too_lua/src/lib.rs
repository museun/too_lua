#![cfg_attr(debug_assertions, allow(dead_code, unused_variables,))]

mod application;
pub use application::Application;

mod script;
pub use script::Script;

#[macro_use]
mod mapping;
pub use mapping::{Binding, Context, Field, Indirect, Mapping};

mod tree;
pub use tree::{DebugNode, Tree};
use tree::{LuaId, Node, UiBuilder};

mod errors;
use errors::Errors;

mod notifications;
use notifications::{Notification, Notifications};

#[macro_use]
mod proxy;
pub use make_class as make_enum;
pub use proxy::{generate, proxy, Proxies, Proxy, ProxyKind, ProxyObject};

pub mod bindings;
#[doc(inline)]
pub use bindings::Bindings;

mod extract;
pub use extract::{merge, Extract};
