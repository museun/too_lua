use too::view::Ui;

use crate::{
    mapping::{Binding, Field},
    params, Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Constrained;

impl Constrained {
    binding! {
        /// Specifically constrain a view
        "constrained" => "constrained" {
            /// The constraint to use
            constraint "Constraint"
        }
    }

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(Ok(constraint)) = ctx.params_field::<params::Constraint>("constraint") else {
            return Mapping::report_missing_data(ui, ctx.id, "constrained", "constraint");
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

        ui.show_children(view, |ui| ctx.visit_children(mapping, ui));
    }
}
