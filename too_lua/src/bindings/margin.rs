use too::view::Ui;

use crate::{mapping::BindingView, Context, Mapping};

make_struct! {
    struct MarginParams is "MarginParams" {
        /// Padding to the left of the view
        left       = Option<u16> ; "integer?"
        /// Padding to the right of the view
        right      = Option<u16> ; "integer?"
        /// Padding to the top of the view
        top        = Option<u16> ; "integer?"
        /// Padding to the bottom of the view
        bottom     = Option<u16> ; "integer?"
        /// Padding on both left and right of the view
        horizontal = Option<u16> ; "integer?"
        /// Padding on both top and bottom of the view
        vertical   = Option<u16> ; "integer?"
        /// Padding on each side of the view
        all        = Option<u16> ; "integer?"
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Margin;

impl BindingView for Margin {
    const SPEC: crate::mapping::BindingSpec = binding! {
        /// Margin applies padding to a view
        "margin" => "MarginParams"
    };
    type Params = MarginParams;
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params::<MarginParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "margin", "params");
        };

        let mut margin = too::math::Margin::new(
            params.left.unwrap_or(0) as i32,
            params.top.unwrap_or(0) as i32,
            params.right.unwrap_or(0) as i32,
            params.bottom.unwrap_or(0) as i32,
        );

        if let Some(horizontal) = params.horizontal {
            margin.left = horizontal as i32;
            margin.right = horizontal as i32;
        }

        if let Some(vertical) = params.vertical {
            margin.top = vertical as i32;
            margin.bottom = vertical as i32;
        }

        if let Some(all) = params.all {
            margin = too::math::Margin::same(all as i32)
        }

        // BUG margin is still weird
        ui.margin(margin, |ui| ctx.visit_children(mapping, ui));
    }
}
