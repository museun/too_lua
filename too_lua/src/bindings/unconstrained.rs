use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, LuaType, Mapping,
};

make_struct! {
    struct UnconstrainedParams is "UnconstrainedParams" {
        /// Unconstrain the horizontal axis
        horizontal = Option<bool> ; "boolean?"
        /// Unconstrain the vertical axis
        vertical = Option<bool> ; "boolean?"
        /// Unconstrain both axis
        both = Option<bool> ; "boolean?"
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Unconstrained;

impl BindingView for Unconstrained {
    const SPEC: BindingSpec = binding! {
        /// Specifically unconstrained a view
        "unconstrained" => UnconstrainedParams::NAME
    };

    type Params = UnconstrainedParams;
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(Ok(table)) = ctx.params_field::<mlua::Table>("constraint") else {
            return Mapping::report_missing_data(ui, ctx.id, "unconstrained", "constraint");
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

        ui.show_children(view, |ui| ctx.visit_children(mapping, ui));
    }
}
