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
        (Aligned::view, Aligned::binding()),
        (Background::view, Background::binding()),
        (Border::view, Border::binding()),
        (Button::view, Button::binding()),
        (Center::view, Center::binding()),
        (Checkbox::view, Checkbox::binding()),
        (Constrained::view, Constrained::binding()),
        (Container::view, Container::binding()),
        (ExpandAxis::view, ExpandAxis::binding()),
        (Fill::view, Fill::binding()),
        (Flex::view, Flex::binding()),
        (Frame::view, Frame::binding()),
        (Horizontal::view, Horizontal::binding()),
        (Label::view, Label::binding()),
        (Margin::view, Margin::binding()),
        (Progress::view, Progress::binding()),
        (Selected::view, Selected::binding()),
        (Separator::view, Separator::binding()),
        (Toggle::view, Toggle::binding()),
        (Slider::view, Slider::binding()),
        (TodoValue::view, TodoValue::binding()),
        (ToggleSwitch::view, ToggleSwitch::binding()),
        (Unconstrained::view, Unconstrained::binding()),
        (Vertical::view, Vertical::binding()),
    ];

    pub fn too_bindings() -> Self {
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
