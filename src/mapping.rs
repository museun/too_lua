use std::collections::HashMap;

use mlua::{AnyUserData, FromLua, UserDataRef, UserDataRefMut};
use slotmap::Key;
use too::{
    format_str,
    helpers::hash_fnv_1a,
    layout::Axis,
    view::{Ui, ViewExt as _},
};

use crate::{
    params::{self, Value},
    LuaId, Node, Tree,
};

#[derive(Copy, Clone)]
pub struct Context<'a> {
    lua: &'a mlua::Lua,
    tree: &'a Tree,
    current: &'a Node,
    id: LuaId,
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

pub type Indirect = fn(&Mapping, &Ui<'_>, Context<'_>);

#[derive(Default)]
pub struct Mapping {
    map: HashMap<u64, Indirect, too::helpers::DefaultIntHasher>,
}

impl Mapping {
    const DEFAULT_TOO_BINDINGS: &[(u64, Indirect)] = &[
        (Self::map_name("vertical"), Self::vertical),
        (Self::map_name("horizontal"), Self::horizontal),
        (Self::map_name("background"), Self::background),
        (Self::map_name("margin"), Self::margin),
        (Self::map_name("separator"), Self::separator),
        (Self::map_name("expand_axis"), Self::expand_axis),
        (Self::map_name("label"), Self::label),
        (Self::map_name("button"), Self::button),
        (Self::map_name("slider"), Self::slider),
        (Self::map_name("progress"), Self::progress),
        (Self::map_name("border"), Self::border),
        (Self::map_name("frame"), Self::frame),
        (Self::map_name("checkbox"), Self::checkbox),
        (Self::map_name("selected"), Self::selected),
        (Self::map_name("todo"), Self::todo_value),
        (Self::map_name("toggle"), Self::toggle_switch),
        (Self::map_name("show"), Self::toggle),
        (Self::map_name("aligned"), Self::aligned),
        (Self::map_name("center"), Self::center),
        (Self::map_name("vertical"), Self::vertical),
        (Self::map_name("horizontal"), Self::horizontal),
        (Self::map_name("wrapped"), Self::wrapped),
        (Self::map_name("flex"), Self::flex),
        (Self::map_name("fill"), Self::fill),
        (Self::map_name("constrained"), Self::constrained),
        (Self::map_name("unconstrained"), Self::unconstrained),
        (Self::map_name("container"), Self::container),
    ];

    pub fn default_bindings() -> Self {
        Self::DEFAULT_TOO_BINDINGS
            .into_iter()
            .fold(Self::default(), |mapping, &(k, v)| mapping.with(k, v))
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
    fn report_missing(ui: &Ui, id: LuaId, hint: &str) {
        let view = too::views::label(format_str!(
            "({:>3?}) {hint}: missing",
            id.data().as_ffi() & 0xffff_ffff
        ))
        .class(too::views::LabelStyle::danger);
        ui.show(view);
    }

    fn report_missing_data(ui: &Ui, id: LuaId, hint: &str, data: &str) {
        let view = too::views::label(format_str!(
            "({:>3?}) {hint}: {data} missing",
            id.data().as_ffi() & 0xffff_ffff
        ))
        .class(too::views::LabelStyle::danger);
        ui.show(view);
    }
}

impl Mapping {
    fn margin(&self, ui: &Ui, ctx: Context) {
        let Some(table) = ctx.tree.map[ctx.id].data.as_table() else {
            return Self::report_missing_data(ui, ctx.id, "margin", "margins");
        };

        let left = table.get::<i32>("left").ok();
        let right = table.get::<i32>("right").ok();
        let top = table.get::<i32>("top").ok();
        let bottom = table.get::<i32>("bottom").ok();

        let horizontal = table.get::<i32>("horizontal").ok();
        let vertical = table.get::<i32>("vertical").ok();
        let all = table.get::<i32>("all").ok();

        let mut margin = too::math::Margin::new(
            left.unwrap_or(0),
            top.unwrap_or(0),
            right.unwrap_or(0),
            bottom.unwrap_or(0),
        );

        if let Some(horizontal) = horizontal {
            margin.left = horizontal;
            margin.right = horizontal;
        }

        if let Some(vertical) = vertical {
            margin.top = vertical;
            margin.bottom = vertical;
        }

        if let Some(all) = all {
            margin = too::math::Margin::same(all)
        }

        ui.margin(margin, |ui| ctx.visit_children(self, ui));
    }

    fn background(&self, ui: &Ui, ctx: Context<'_>) {
        let Some(Ok(bg)) = ctx.params_field::<params::Color>("background") else {
            return Self::report_missing_data(ui, ctx.id, "background", "bg");
        };

        ui.background(bg.0, |ui| ctx.visit_children(self, ui));
    }

    fn aligned(&self, ui: &Ui, ctx: Context) {
        let Some(Ok(aligned)) = ctx.params_field::<params::Aligned>("align") else {
            return Self::report_missing(ui, ctx.id, "aligned");
        };

        use too::layout::Align2;
        let align = match aligned {
            params::Aligned::LeftTop => Align2::LEFT_TOP,
            params::Aligned::CenterTop => Align2::CENTER_TOP,
            params::Aligned::RightTop => Align2::RIGHT_TOP,
            params::Aligned::LeftCenter => Align2::LEFT_CENTER,
            params::Aligned::CenterCenter => Align2::CENTER_CENTER,
            params::Aligned::RightCenter => Align2::RIGHT_CENTER,
            params::Aligned::LeftBottom => Align2::LEFT_BOTTOM,
            params::Aligned::CenterBottom => Align2::CENTER_BOTTOM,
            params::Aligned::RightBottom => Align2::RIGHT_BOTTOM,
        };

        ui.aligned(align, |ui| ctx.visit_children(self, ui));
    }

    fn separator(&self, ui: &Ui, _ctx: Context) {
        // TODO this can be styled
        ui.separator();
    }

    fn expand_axis(&self, ui: &Ui, _ctx: Context) {
        ui.expand_axis();
    }

    fn label(&self, ui: &Ui, ctx: Context) {
        if let Some(text) = ctx.text() {
            ui.label(&text);
            return;
        }

        let Some(Ok(text)) = ctx.params_field::<String>("text") else {
            return Self::report_missing_data(ui, ctx.id, "label", "text");
        };

        let Ok(params) = ctx.params::<params::LabelParams>() else {
            return Self::report_missing_data(ui, ctx.id, "label", "params");
        };

        use too::views::Label as L;
        type Apply = fn(L) -> L;

        let mut label = too::views::label(text);

        let mut fg = None;
        if let Some(style) = params.style {
            if let Some(params::Color(new)) = style.foreground {
                fg = Some(new)
            }

            label = [
                style.italic,
                style.bold,
                style.underline,
                style.faint,
                style.blink,
                style.strikeout,
            ]
            .into_iter()
            .map(|c| c.unwrap_or_default())
            .zip([
                L::italic as Apply,
                L::bold,
                L::underline,
                L::faint,
                L::blink,
                L::strikeout,
            ])
            .filter_map(|(v, a)| v.then_some(a))
            .fold(label, |l, a| a(l))
        }

        let class = params
            .class
            .and_then(|class| {
                let val = match class {
                    params::LabelClass::Info => too::views::LabelStyle::info,
                    params::LabelClass::Warning => too::views::LabelStyle::warning,
                    params::LabelClass::Success => too::views::LabelStyle::success,
                    params::LabelClass::Danger => too::views::LabelStyle::danger,
                    _ => return None,
                };
                Some(val)
            })
            .unwrap_or(<too::views::LabelStyle as too::view::Style>::default);

        let label = if let Some(fg) = fg {
            label.fg(fg)
        } else {
            label.class(class)
        };
        ui.show(label);
    }

    fn button(&self, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<params::ButtonParams>() else {
            return Self::report_missing_data(ui, ctx.id, "button", "params");
        };

        let Some(Ok(text)) = ctx.params_field::<String>("text") else {
            return Self::report_missing_data(ui, ctx.id, "button", "text");
        };

        let Some(Ok(handler)) = ctx.params_field::<mlua::Function>("handler") else {
            return Self::report_missing_data(ui, ctx.id, "button", "handler");
        };

        let view = too::views::button(text).class(params.apply());

        if ui.show(view).clicked() {
            let _ = handler.call::<()>(());
        }
    }

    fn slider(&self, ui: &Ui, ctx: Context) {
        let params = ctx.params::<params::SliderParams>();
        let axis = ctx.axis();

        let Some(mut v) = ctx.value_mut() else {
            return Self::report_missing(ui, ctx.id, "slider");
        };
        let Value::Float(v) = &mut *v else {
            return Self::report_missing(ui, ctx.id, "float value");
        };

        let mut view = too::views::slider(v).axis(axis);
        if let Ok(params) = params {
            view = view.class(params.apply())
        }
        ui.show(view);
    }

    fn progress(&self, ui: &Ui, ctx: Context) {
        let params = ctx.params::<params::ProgressParams>();
        let axis = ctx.axis();

        let Some(value) = ctx.value() else {
            return Self::report_missing(ui, ctx.id, "progress");
        };
        let Value::Float(value) = *value else {
            return Self::report_missing(ui, ctx.id, "float value");
        };

        let mut view = too::views::progress(value).axis(axis);
        if let Ok(params) = params {
            view = view.class(params.apply())
        }
        ui.show(view);
    }

    fn checkbox(&self, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<params::CheckboxParams>() else {
            return Self::report_missing_data(ui, ctx.id, "checkbox", "params");
        };

        let Some(Ok(text)) = ctx.params_field::<String>("text") else {
            return Self::report_missing_data(ui, ctx.id, "checkbox", "text");
        };

        let Some(mut value) = ctx.value_mut() else {
            return Self::report_missing_data(ui, ctx.id, "checkbox", "value");
        };

        let Value::Bool(value) = &mut *value else {
            return Self::report_missing(ui, ctx.id, "bool value");
        };

        ui.show(too::views::checkbox(value, text).class(params.apply()));
    }

    fn selected(&self, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<params::SelectedParams>() else {
            return Self::report_missing_data(ui, ctx.id, "checkbox", "params");
        };

        let Some(Ok(text)) = ctx.params_field::<String>("text") else {
            return Self::report_missing_data(ui, ctx.id, "selected", "text");
        };

        let Some(mut value) = ctx.value_mut() else {
            return Self::report_missing_data(ui, ctx.id, "selected", "value");
        };

        let Value::Bool(value) = &mut *value else {
            return Self::report_missing(ui, ctx.id, "bool value");
        };

        ui.show(too::views::selected(value, text).class(params.apply()));
    }

    fn todo_value(&self, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<params::TodoParams>() else {
            return Self::report_missing_data(ui, ctx.id, "checkbox", "params");
        };

        let Some(Ok(text)) = ctx.params_field::<String>("text") else {
            return Self::report_missing_data(ui, ctx.id, "todo", "text");
        };

        let Some(mut value) = ctx.value_mut() else {
            return Self::report_missing_data(ui, ctx.id, "todo", "value");
        };

        let Value::Bool(value) = &mut *value else {
            return Self::report_missing(ui, ctx.id, "bool value");
        };

        let view = too::views::todo_value(value, text);
        let class = params
            .class
            .and_then(|class| {
                #[allow(unreachable_patterns)]
                let val = match class {
                    params::TodoClass::Default => {
                        <too::views::TodoStyle as too::view::Style>::default
                    }
                    _ => return None,
                };
                Some(val)
            })
            .unwrap_or(<too::views::TodoStyle as too::view::Style>::default);

        let mut attr = None;
        let mut text_color = None;
        let mut hovered_color = None;

        use too::renderer::Attribute;
        if let Some(style) = params.style {
            let new = [
                (style.italic, Attribute::ITALIC),
                (style.bold, Attribute::BOLD),
                (style.underline, Attribute::UNDERLINE),
                (style.faint, Attribute::FAINT),
                (style.blink, Attribute::BLINK),
                (style.strikeout, Attribute::STRIKEOUT),
            ]
            .into_iter()
            .filter_map(|(c, a)| c.unwrap_or_default().then_some(a))
            .fold(Attribute::RESET, |l, a| l | a);

            attr = Some(new).filter(|c| !c.is_reset());
            text_color = style.text_color.map(|c| c.0);
            hovered_color = style.hovered_color.map(|c| c.0);
        }

        ui.show(view.class(move |palette, options| {
            let mut style = class(palette, options);
            if let Some(attr) = attr {
                style.selected = attr;
            }
            if let Some(text_color) = text_color {
                style.text_color = text_color;
            }
            if let Some(hovered_color) = hovered_color {
                style.hovered_color = Some(hovered_color);
            }
            style
        }));
    }

    fn toggle_switch(&self, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<params::ToggleParams>() else {
            return Self::report_missing_data(ui, ctx.id, "toggle", "params");
        };

        let Some(mut value) = ctx.value_mut() else {
            return Self::report_missing_data(ui, ctx.id, "toggle", "value");
        };

        let Value::Bool(value) = &mut *value else {
            return Self::report_missing(ui, ctx.id, "bool value");
        };

        let axis = ctx.axis();

        ui.show(
            too::views::toggle_switch(value)
                .axis(axis)
                .class(params.apply()),
        );
    }

    fn toggle(&self, ui: &Ui, ctx: Context) {
        let Some(value) = ctx.value() else {
            return Self::report_missing_data(ui, ctx.id, "show", "value");
        };

        let Value::Bool(value) = *value else {
            return Self::report_missing(ui, ctx.id, "bool value");
        };

        ui.toggle(value, |ui| ctx.visit_children(self, ui));
    }

    fn border(&self, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<params::BorderParams>() else {
            return Self::report_missing_data(ui, ctx.id, "border", "params");
        };

        let Some(Ok(border)) = ctx.params_field::<params::Border>("border") else {
            return Self::report_missing_data(ui, ctx.id, "border", "border");
        };

        use too::renderer::Border;
        let border = match border {
            params::Border::Empty => Border::EMPTY,
            params::Border::Thin => Border::THIN,
            params::Border::ThinWide => Border::THIN_WIDE,
            params::Border::Rounded => Border::ROUNDED,
            params::Border::Double => Border::DOUBLE,
            params::Border::Thick => Border::THICK,
            params::Border::ThickTall => Border::THICK_TALL,
            params::Border::ThickWide => Border::THICK_WIDE,
        };

        let view = too::views::border(border).class(params.apply());
        ui.show_children(view, |ui| ctx.visit_children(self, ui));
    }

    fn frame(&self, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<params::BorderParams>() else {
            return Self::report_missing_data(ui, ctx.id, "frame", "params");
        };
        let Some(Ok(border)) = ctx.params_field::<params::Border>("border") else {
            return Self::report_missing_data(ui, ctx.id, "frame", "border");
        };
        let Some(Ok(title)) = ctx.params_field::<String>("title") else {
            return Self::report_missing_data(ui, ctx.id, "frame", "title");
        };

        let align = ctx
            .params_field::<params::Align>("align")
            .transpose()
            .ok()
            .flatten()
            .unwrap_or(params::Align::Middle);

        use too::renderer::Border;
        let border = match border {
            params::Border::Empty => Border::EMPTY,
            params::Border::Thin => Border::THIN,
            params::Border::ThinWide => Border::THIN_WIDE,
            params::Border::Rounded => Border::ROUNDED,
            params::Border::Double => Border::DOUBLE,
            params::Border::Thick => Border::THICK,
            params::Border::ThickTall => Border::THICK_TALL,
            params::Border::ThickWide => Border::THICK_WIDE,
        };

        let view = too::views::frame(border, title).title_align(match align {
            params::Align::Min => too::layout::Align::Min,
            params::Align::Middle => too::layout::Align::Center,
            params::Align::Max => too::layout::Align::Max,
        });

        ui.show_children(view.class(params.apply()), |ui| {
            ctx.visit_children(self, ui)
        });
    }

    fn center(&self, ui: &Ui, ctx: Context) {
        ui.center(|ui| ctx.visit_children(self, ui));
    }

    fn flex(&self, ui: &Ui, ctx: Context) {
        use too::layout::Flex;
        if let Some(Ok(factor)) = ctx.params_field::<f32>("tight") {
            ui.show_children(too::views::Flexible::new(Flex::Tight(factor)), |ui| {
                ctx.visit_children(self, ui)
            });

            return;
        };

        let factor = ctx.params_field_opt("loose").unwrap_or(1.0);
        ui.show_children(too::views::Flexible::new(Flex::Loose(factor)), |ui| {
            ctx.visit_children(self, ui)
        });
    }

    fn fill(&self, ui: &Ui, ctx: Context) {
        let Some(Ok(params::Color(rgba))) = ctx.params_field::<params::Color>("background") else {
            return Self::report_missing_data(ui, ctx.id, "fill", "background");
        };

        if let Some(Ok(table)) = ctx.params_field::<mlua::Table>("space") {
            let Ok(width) = table.get::<u16>("width") else {
                return Self::report_missing_data(ui, ctx.id, "fill", "space.width");
            };
            let Ok(height) = table.get::<u16>("height") else {
                return Self::report_missing_data(ui, ctx.id, "fill", "space.height");
            };
            ui.show(too::views::Fill::new(rgba, (width as i32, height as i32)));
        } else {
            ui.show(too::views::Fill::fill_with(rgba));
        }
    }

    fn constrained(&self, ui: &Ui, ctx: Context) {
        let Some(Ok(constraint)) = ctx.params_field::<params::Constraint>("constraint") else {
            return Self::report_missing_data(ui, ctx.id, "constrained", "constraint");
        };

        use params::Constraint::*;
        use too::views::Constrain;
        let view = match constraint {
            ExactSize { w, h } => Constrain::exact_size((w as i32, h as i32)),
            MaxSize { w, h } => Constrain::max_size((w as i32, h as i32)),
            MinSize { w, h } => Constrain::min_size((w as i32, h as i32)),
            ExactHeight(v) => Constrain::exact_height(v as i32),
            ExactWidth(v) => Constrain::exact_width(v as i32),
            MaxHeight(v) => Constrain::max_height(v as i32),
            MaxWidth(v) => Constrain::max_width(v as i32),
            MinWidth(v) => Constrain::min_width(v as i32),
            MinHeight(v) => Constrain::min_height(v as i32),
        };

        ui.show_children(view, |ui| ctx.visit_children(self, ui));
    }

    fn unconstrained(&self, ui: &Ui, ctx: Context) {
        let Some(Ok(table)) = ctx.params_field::<mlua::Table>("constraint") else {
            return Self::report_missing_data(ui, ctx.id, "unconstrained", "constraint");
        };

        let horizontal = table.get::<bool>("horizontal").unwrap_or_default();
        let vertical = table.get::<bool>("vertical").unwrap_or_default();
        let both = table.get::<bool>("both").unwrap_or_default();

        let view = if both {
            too::views::Unconstrained::both()
        } else {
            too::views::Unconstrained::new()
                .horizontal(horizontal)
                .vertical(vertical)
        };

        ui.show_children(view, |ui| ctx.visit_children(self, ui));
    }

    fn vertical(&self, ui: &Ui, ctx: Context) {
        Self::_list(self, ui, ctx, Axis::Vertical);
    }

    fn horizontal(&self, ui: &Ui, ctx: Context) {
        Self::_list(self, ui, ctx, Axis::Horizontal);
    }

    fn _list(self: &Mapping, ui: &Ui, ctx: Context, axis: Axis) {
        let mut list = too::views::list().axis(axis);

        if let Some(justify) = ctx.params_field_opt::<params::Justify>("justify") {
            let justify = match justify {
                params::Justify::Start => too::layout::Justify::Start,
                params::Justify::End => too::layout::Justify::End,
                params::Justify::Center => too::layout::Justify::Center,
                params::Justify::SpaceBetween => too::layout::Justify::SpaceBetween,
                params::Justify::SpaceAround => too::layout::Justify::SpaceAround,
                params::Justify::SpaceEvenly => too::layout::Justify::SpaceEvenly,
            };
            list = list.justify(justify);
        }

        if let Some(cross_align) = ctx.params_field_opt::<params::CrossAlign>("cross_align") {
            let cross_align = match cross_align {
                params::CrossAlign::Start => too::layout::CrossAlign::Start,
                params::CrossAlign::End => too::layout::CrossAlign::End,
                params::CrossAlign::Center => too::layout::CrossAlign::Center,
                params::CrossAlign::Stretch => too::layout::CrossAlign::Stretch,
                params::CrossAlign::Fill => too::layout::CrossAlign::Fill,
            };
            list = list.cross_align(cross_align);
        }

        if let Some(gap) = ctx.params_field_opt::<u16>("gap") {
            list = list.gap(gap as i32)
        }

        if let Some(scrollable) = ctx.params_field_opt::<bool>("scrollable") {
            list = list.scrollable(scrollable)
        }

        ui.show_children(list, |ui| ctx.visit_children(self, ui));
    }

    fn wrapped(&self, ui: &Ui, ctx: Context) {
        // TODO params
        let data = &ctx.tree.map[ctx.id].data;
        let mode = if let Some(table) = data.as_table() {
            if table.get::<bool>("horizontal").unwrap_or(true) {
                Axis::Horizontal
            } else {
                Axis::Vertical
            }
        } else {
            Axis::Horizontal
        };

        too::views::Wrap::new(mode).gap(1).show_children(ui, |ui| {
            for &child in &ctx.tree.map[ctx.id].children {
                self.evaluate(ui, ctx.child(child));
            }
        });
    }

    fn container(&self, ui: &Ui, ctx: Context) {
        ctx.visit_children(self, ui);
    }
}
