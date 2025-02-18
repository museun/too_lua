use mlua::{AnyUserData, FromLua, UserDataRef, UserDataRefMut};
use too::view::Ui;

use crate::{bindings::Value, LuaId, Node, Tree};

use super::Mapping;

#[derive(Copy, Clone)]
pub struct Context<'a> {
    pub(crate) lua: &'a mlua::Lua,
    pub(crate) tree: &'a Tree,
    pub(crate) current: &'a Node,
    pub(crate) id: LuaId,
}

impl<'a> Context<'a> {
    pub(crate) const fn new(
        lua: &'a mlua::Lua,
        tree: &'a Tree,
        current: &'a Node,
        id: LuaId,
    ) -> Self {
        Self {
            lua,
            tree,
            current,
            id,
        }
    }

    pub(crate) fn child(self, id: LuaId) -> Self {
        Self {
            lua: self.lua,
            tree: self.tree,
            current: &self.tree.map[id],
            id,
        }
    }

    #[inline(always)]
    pub fn visit_children(&self, mapping: &Mapping, ui: &Ui) {
        for &child in &self.tree.map[self.id].children {
            mapping.evaluate(ui, self.child(child));
        }
    }

    pub fn foo<T: FromLua>(&self) -> Option<T> {
        T::from_lua(self.current.data.clone(), &self.lua).ok()
    }

    pub fn value_ref(&self, data: &AnyUserData) -> Option<UserDataRef<Value>> {
        data.borrow::<Value>().ok()
    }

    pub fn value_mut(&self, data: &AnyUserData) -> Option<UserDataRefMut<Value>> {
        data.borrow_mut::<Value>().ok()
    }
}
