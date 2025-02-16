use too::view::Ui;

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Margin;

impl Margin {
    binding! {
        /// Margin applies padding to a view
        "margin" => "margin" {
            /// Padding to the left of the view
            left "integer?"
            /// Padding to the right of the view
            right "integer?"
            /// Padding to the top of the view
            top "integer?"
            /// Padding to the bottom of the view
            bottom "integer?"
            /// Padding on both left and right of the view
            horizontal "integer?"
            /// Padding on both top and bottom of the view
            vertical "integer?"
            /// Padding on each side of the view
            all "integer?"
        }
    }

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(table) = ctx.tree.map[ctx.id].data.as_table() else {
            return Mapping::report_missing_data(ui, ctx.id, "margin", "margins");
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

        ui.margin(margin, |ui| ctx.visit_children(mapping, ui));
    }
}
