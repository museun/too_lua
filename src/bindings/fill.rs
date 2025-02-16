use too::view::Ui;

use crate::{
    mapping::{Binding, Field},
    params, Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Fill;

impl Fill {
    binding! {
        /// Fill the childrens area, with an optional size constraint
        "fill" => "fill" {
            /// Use this color to fill the area
            background "string"
            /// Optional space to allocate
            space "{width: integer?, height: integer?}"
        }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(Ok(params::Color(rgba))) = ctx.params_field::<params::Color>("background") else {
            return Mapping::report_missing_data(ui, ctx.id, "fill", "background");
        };

        if let Some(Ok(table)) = ctx.params_field::<mlua::Table>("space") {
            let Ok(width) = table.get::<u16>("width") else {
                return Mapping::report_missing_data(ui, ctx.id, "fill", "space.width");
            };
            let Ok(height) = table.get::<u16>("height") else {
                return Mapping::report_missing_data(ui, ctx.id, "fill", "space.height");
            };
            ui.show(too::views::Fill::new(rgba, (width as i32, height as i32)));
        } else {
            ui.show(too::views::Fill::fill_with(rgba));
        }
    }
}
