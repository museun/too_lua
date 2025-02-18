use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, LuaType, Mapping,
};

use super::Color;

crate::make_struct! {
    struct FillParams is "FillParams" {
        /// Use this color to fill the area
        background = Color       ; "string"
        /// Optional width to allocate
        width      = Option<u16> ; "integer?"
        /// Optional height to allocate
        height     = Option<u16> ; "integer?"
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Fill;

impl BindingView for Fill {
    const SPEC: BindingSpec = binding! {
        /// Fill the childrens area, with an optional size constraint
        "fill" => FillParams::NAME
    };

    type Params = FillParams;
    type Style = ();

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(Ok(color)) = ctx.params_field::<Color>("background") else {
            return Mapping::report_missing_data(ui, ctx.id, "fill", "background");
        };

        if let Some(Ok(table)) = ctx.params_field::<mlua::Table>("space") {
            let Ok(width) = table.get::<u16>("width") else {
                return Mapping::report_missing_data(ui, ctx.id, "fill", "space.width");
            };
            let Ok(height) = table.get::<u16>("height") else {
                return Mapping::report_missing_data(ui, ctx.id, "fill", "space.height");
            };
            ui.show(too::views::Fill::new(color, (width as i32, height as i32)));
        } else {
            ui.show(too::views::Fill::fill_with(color));
        }
    }
}
