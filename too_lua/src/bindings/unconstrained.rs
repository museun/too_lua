use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, Mapping,
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
        "unconstrained" => "UnconstrainedParams"
    };

    type Params = UnconstrainedParams;
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.foo::<UnconstrainedParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "unconstrained", "params");
        };

        let both = params.both.unwrap_or_default();
        let horizontal = params.horizontal.unwrap_or_default();
        let vertical = params.vertical.unwrap_or_default();

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
