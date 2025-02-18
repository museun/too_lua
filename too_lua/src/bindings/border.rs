use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    bindings::Color,
    mapping::{BindingSpec, BindingView},
    proxy::Params,
    Context, LuaType, Mapping,
};

make_enum! {
    enum BorderKind is "BorderKind" {
        /// No border
        Empty     = "empty"
        /// A thin border
        Thin      = "thin"
        /// A thin, but wide border
        ThinWide  = "thin_wide"
        /// A rounded border
        Rounded   = "rounded"
        /// A double-line border
        Double    = "double"
        /// A thick border
        Thick     = "thick"
        /// A thick, but tall border
        ThickTall = "thick_tall"
        /// A thick, but wide border
        ThickWide = "thick_wide"
    }
}

make_class! {
    class BorderClass is "Border"    ; too::views::BorderStyle {
        /// The default style
        Default      = "default"     ; too::views::BorderStyle::default
        /// An interactive style
        Interactive  = "interactive" ; too::views::BorderStyle::interactive
    }
}

make_style! {
    style BorderStyle is "BorderStyle" ; too::views::BorderStyle {
        /// The frame title color
        title          = Option<Color> ; "Color?"
        /// The color of the border
        border         = Option<Color> ; "Color?"
        /// The color of the border, when focused
        border_focused = Option<Color> ; "Color?"
        /// The color of the border, when hovered
        border_hovered = Option<Color> ; "Color?"
    }
}

make_struct! {
    struct BorderParams is "BorderParams" {
        /// The style of the border
        style  = Option<BorderStyle> ; "BorderStyle?"
        /// The class of the border
        class  = Option<BorderClass> ; "Border?"
        /// The border to use
        border = BorderKind          ; "BorderKind"
    }
}

impl Params<too::views::BorderStyle> for BorderParams {
    type Class = BorderClass;
    type Style = BorderStyle;

    fn class(&self) -> &Option<Self::Class> {
        &self.class
    }
    fn style(&self) -> &Option<Self::Style> {
        &self.style
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Border;

impl BindingView for Border {
    const SPEC: BindingSpec = binding! {
        /// Border to surround its children
        "border" => BorderParams::NAME
    };

    type Params = BorderParams;
    type Style = BorderStyle;

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<BorderParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "border", "params");
        };

        let Some(Ok(border)) = ctx.params_field::<BorderKind>("border") else {
            return Mapping::report_missing_data(ui, ctx.id, "border", "border");
        };

        use too::renderer::Border;
        let border = match border {
            BorderKind::Empty => Border::EMPTY,
            BorderKind::Thin => Border::THIN,
            BorderKind::ThinWide => Border::THIN_WIDE,
            BorderKind::Rounded => Border::ROUNDED,
            BorderKind::Double => Border::DOUBLE,
            BorderKind::Thick => Border::THICK,
            BorderKind::ThickTall => Border::THICK_TALL,
            BorderKind::ThickWide => Border::THICK_WIDE,
        };

        let view = too::views::border(border).class(params.apply_styling());

        ui.show_children(view, |ui| ctx.visit_children(mapping, ui));
    }
}
