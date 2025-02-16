use std::collections::HashMap;

use slotmap::Key;
use too::{
    format_str,
    helpers::hash_fnv_1a,
    view::{Ui, ViewExt as _},
};

use crate::LuaId;

mod context;
pub use context::Context;

#[macro_use]
mod binding;
pub use binding::{Binding, Field};

use crate::bindings::{
    Aligned, Background, Border, Button, Center, Checkbox, Constrained, Container, ExpandAxis,
    Fill, Flex, Frame, Horizontal, Label, Margin, Progress, Selected, Separator, Slider, TodoValue,
    Toggle, ToggleSwitch, Unconstrained, Vertical,
};

pub type Indirect = fn(&Mapping, &Ui<'_>, Context<'_>);

#[derive(Default)]
pub struct Mapping {
    map: HashMap<u64, Indirect, too::helpers::DefaultIntHasher>,
}

impl Mapping {
    pub(crate) const DEFAULT_TOO_BINDINGS: &[(Indirect, Binding)] = &[
        (Aligned::view, Aligned::BINDING),
        (Background::view, Background::BINDING),
        (Border::view, Border::BINDING),
        (Button::view, Button::BINDING),
        (Center::view, Center::BINDING),
        (Checkbox::view, Checkbox::BINDING),
        (Constrained::view, Constrained::BINDING),
        (Container::view, Container::BINDING),
        (ExpandAxis::view, ExpandAxis::BINDING),
        (Fill::view, Fill::BINDING),
        (Flex::view, Flex::BINDING),
        (Frame::view, Frame::BINDING),
        (Horizontal::view, Horizontal::BINDING),
        (Label::view, Label::BINDING),
        (Margin::view, Margin::BINDING),
        (Progress::view, Progress::BINDING),
        (Selected::view, Selected::BINDING),
        (Separator::view, Separator::BINDING),
        (Toggle::view, Toggle::BINDING),
        (Slider::view, Slider::BINDING),
        (TodoValue::view, TodoValue::BINDING),
        (ToggleSwitch::view, ToggleSwitch::BINDING),
        (Unconstrained::view, Unconstrained::BINDING),
        (Vertical::view, Vertical::BINDING),
    ];

    pub fn default_bindings() -> Self {
        Self::DEFAULT_TOO_BINDINGS
            .iter()
            .fold(Self::default(), |mapping, &(func, binding)| {
                mapping.with(Self::map_name(binding.name), func)
            })
    }

    pub fn evaluate(&self, ui: &Ui, ctx: Context<'_>) {
        if ctx.id == ctx.tree.root {
            ctx.visit_children(self, ui);
            return;
        }

        let name = &ctx.tree.map[ctx.id].name;

        let Some(func) = self.map.get(name) else {
            ui.label(format_str!(
                "cannot find: {name}/{id:?}",
                name = &ctx.tree.names[ctx.id],
                id = ctx.id.data()
            ));
            return;
        };

        func(self, ui, ctx);
    }

    pub fn with(mut self, view: u64, value: Indirect) -> Self {
        self.map.insert(view, value);
        self
    }

    pub const fn map_name(name: &str) -> u64 {
        hash_fnv_1a(name.as_bytes())
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
