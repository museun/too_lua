use mlua::{AnyUserData, FromLua, UserDataRef, UserDataRefMut};
use too::{layout::Axis, view::Ui};

use crate::{
    params::{self, Value},
    LuaId, Node, Tree,
};

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

    pub fn visit_children(&self, mapping: &Mapping, ui: &Ui) {
        for &child in &self.tree.map[self.id].children {
            mapping.evaluate(ui, self.child(child));
        }
    }

    pub fn axis(&self) -> Axis {
        match self
            .params_field_opt("axis")
            .unwrap_or(params::Axis::Horizontal)
        {
            params::Axis::Vertical => Axis::Vertical,
            params::Axis::Horizontal => Axis::Horizontal,
        }
    }

    pub fn params_field_opt<T>(&self, key: &str) -> Option<T>
    where
        T: FromLua,
    {
        self.params_field(key).transpose().ok().flatten()
    }

    pub fn params_field_default<T>(&self, key: &str) -> T
    where
        T: FromLua + Default,
    {
        self.params_field_opt(key).unwrap_or_default()
    }

    pub fn params_field<T>(&self, key: &str) -> Option<mlua::Result<T>>
    where
        T: FromLua,
    {
        if let Some(table) = self.current.data.as_table() {
            return Some(table.get(key));
        }
        None
    }

    pub fn params<T>(&self) -> mlua::Result<T>
    where
        T: FromLua,
    {
        T::from_lua(self.current.data.clone(), self.lua)
    }

    pub fn text(&self) -> Option<String> {
        self.current.data.as_string_lossy()
    }

    pub fn value(&self) -> Option<UserDataRef<Value>> {
        Self::find(&self.current.data).and_then(|c| c.borrow().ok())
    }

    pub fn value_mut(&self) -> Option<UserDataRefMut<Value>> {
        Self::find(&self.current.data).and_then(|c| c.borrow_mut().ok())
    }

    fn find(value: &mlua::Value) -> Option<AnyUserData> {
        match value {
            mlua::Value::Table(table) => table.get("value").or_else(|_| table.get(1)).ok(),
            mlua::Value::UserData(ud) => Some(ud.clone()),
            _ => None,
        }
    }
}
