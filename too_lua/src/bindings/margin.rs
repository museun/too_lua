use anno_lua::Anno;
use too::view::Ui;

use crate::{Context, Mapping, None, Spec, View};

#[derive(Copy, Clone, Debug, PartialEq, Anno, serde::Deserialize)]
#[anno(exact)]
pub struct MarginParams {
    /// Padding to the left of the view
    #[anno(lua_type = "integer?")]
    pub left: Option<u16>,

    /// Padding to the right of the view
    #[anno(lua_type = "integer?")]
    pub right: Option<u16>,

    /// Padding to the top of the view
    #[anno(lua_type = "integer?")]
    pub top: Option<u16>,

    /// Padding to the bottom of the view
    #[anno(lua_type = "integer?")]
    pub bottom: Option<u16>,

    /// Padding on both left and right of the view
    #[anno(lua_type = "integer?")]
    pub horizontal: Option<u16>,

    /// Padding on both top and bottom of the view
    #[anno(lua_type = "integer?")]
    pub vertical: Option<u16>,

    /// Padding on each side of the view
    #[anno(lua_type = "integer?")]
    pub all: Option<u16>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Margin;

impl View for Margin {
    type Params = MarginParams;
    type Style = None;

    fn spec() -> Spec {
        view_spec! {
            /// Margin applies padding to a view
            Self {
                name: "margin",
                params: "MarginParams"
            }
        }
    }
    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params_de::<MarginParams>() else {
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
