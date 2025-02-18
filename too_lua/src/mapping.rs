use std::collections::HashMap;

use slotmap::Key;
use too::{
    format_str,
    helpers::hash_fnv_1a,
    view::{Ui, ViewExt as _},
};

use crate::{Bindings, LuaId};

mod context;
pub use context::Context;

#[macro_use]
mod binding;
pub use binding::{Binding, BindingArgs, BindingParams, BindingSpec, BindingView};

pub type Indirect = fn(&Mapping, &Ui<'_>, Context<'_>);

#[derive(Default)]
pub struct Mapping {
    map: HashMap<u64, Indirect, too::helpers::DefaultIntHasher>,
}

impl Mapping {
    pub fn from_bindings(bindings: Bindings) -> Self {
        Self::default().with_many(
            bindings
                .into_iter()
                .map(|&(binding, func)| (binding.name, func)),
        )
    }

    pub fn with_many<'s>(self, many: impl IntoIterator<Item = (&'s str, Indirect)>) -> Self {
        many.into_iter().fold(self, |mapping, (name, func)| {
            mapping.with(Self::map_name(name), func)
        })
    }

    pub fn with(mut self, view: u64, value: Indirect) -> Self {
        self.map.insert(view, value);
        self
    }

    pub const fn map_name(name: &str) -> u64 {
        hash_fnv_1a(name.as_bytes())
    }

    #[inline(always)]
    pub fn evaluate(&self, ui: &Ui, ctx: Context<'_>) {
        if ctx.id == ctx.tree.root {
            ctx.visit_children(self, ui);
            return;
        }

        let name = &ctx.tree.map[ctx.id].name;
        let Some(func) = self.map.get(name) else {
            ui.label(format_str!(
                "cannot find: {name}/{id:?}",
                name = &ctx.tree.names[ctx.id].to_string_lossy(),
                id = ctx.id.data()
            ));
            return;
        };

        func(self, ui, ctx);
    }
}

impl Mapping {
    pub fn report_missing(ui: &Ui, id: LuaId, hint: &str) {
        let view = too::views::label(format_str!(
            "({:>3?}) {hint}: missing",
            id.data().as_ffi() & 0xffff_ffff
        ))
        .class(too::views::LabelStyle::danger);
        ui.show(view);
    }

    pub fn report_missing_data(ui: &Ui, id: LuaId, hint: &str, data: &str) {
        let view = too::views::label(format_str!(
            "({:>3?}) {hint}: {data} missing",
            id.data().as_ffi() & 0xffff_ffff
        ))
        .class(too::views::LabelStyle::danger);
        ui.show(view);
    }
}
