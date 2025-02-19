use mlua::AnyUserData;
use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    mapping::{BindingSpec, BindingView},
    proxy::Params,
    Context, Mapping,
};

use super::{Axis, Color};

make_class! {
    class ProgressClass is "Progress" ; too::views::ProgressStyle {
        /// Default style
        Default      = "default"       ; too::views::ProgressStyle::default
        /// A medium filled style
        MediumFilled = "medium_filled" ; too::views::ProgressStyle::medium_filled
        /// A full filled style
        Filled       = "filled"        ; too::views::ProgressStyle::filled
        /// A thin style
        Thin         = "thin"          ; too::views::ProgressStyle::thin
        /// A thick style
        Thick        = "thick"         ; too::views::ProgressStyle::thick
        /// A thin, but dashed style
        ThinDashed   = "thin_dashed"   ; too::views::ProgressStyle::thin_dashed
        /// A thick, but dashed style
        ThickDashed  = "thick_dashed"  ; too::views::ProgressStyle::thick_dashed
    }
}

make_style! {
    style ProgressStyle is "ProgressStyle" ; too::views::ProgressStyle {
        /// The unfilled color
        unfilled_color   = Option<Color>  ; "Color?"
        /// The filled color
        filled_color     = Option<Color>  ; "Color?"
        /// The unfilled color, when hovered
        unfilled_hovered = Option<Color>  ; "Color?"
        /// The filled color, when hovered
        filled_hovered   = Option<Color>  ; "Color?"
        /// The character to use for the unfilled portion
        unfilled         = Option<String> ; "string?"
        /// The character to use for the filled portion
        filled           = Option<String> ; "string?"
    }
}

make_struct! {
    struct ProgressParams is "ProgressParams" {
        /// The style of the progress bar
        style = Option<ProgressStyle> ; "ProgressStyle?"
        /// The class of the progress bar
        class = Option<ProgressClass> ; "Progress?"
        /// Axis to use for layout
        axis  = Option<Axis>          ; "Axis?"
        /// The value to use (an f32 in the range of 0.0 ..= 1.0)
        value = AnyUserData           ; "Value"
    }
}

impl Params<too::views::ProgressStyle> for ProgressParams {
    type Class = ProgressClass;
    type Style = ProgressStyle;

    fn class(&self) -> &Option<Self::Class> {
        &self.class
    }
    fn style(&self) -> &Option<Self::Style> {
        &self.style
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Progress;

impl BindingView for Progress {
    const SPEC: BindingSpec = binding! {
        /// A progress bar
        "progress" => "ProgressParams"
    };

    type Params = ProgressParams;
    type Style = ProgressStyle;

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params::<ProgressParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "progress", "params");
        };

        let Some(value) = ctx.value_ref(&params.value) else {
            return Mapping::report_missing_data(ui, ctx.id, "progress", "value");
        };

        let Some(value) = value.float_ref() else {
            return Mapping::report_missing_data(ui, ctx.id, "progress", "float");
        };

        let axis = params.axis.unwrap_or_default();
        let view = too::views::progress(*value)
            .axis(axis.into())
            .class(params.apply_styling());

        ui.show(view);
    }
}
