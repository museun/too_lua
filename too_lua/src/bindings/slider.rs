use mlua::AnyUserData;
use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    mapping::{BindingSpec, BindingView},
    proxy::Params,
    Context, Mapping,
};

use super::{Axis, Color};

make_class! {
    class SliderClass is "Slider" ; too::views::SliderStyle {
        /// The default style
        Default      = "default"       ; too::views::SliderStyle::default
        /// Small track and rounded knob
        SmallRounded = "small_rounded" ; too::views::SliderStyle::small_rounded
        /// Small track and diamond knob
        SmallDiamond = "small_diamond" ; too::views::SliderStyle::small_diamond
        /// Small track and square knob
        SmallSquare  = "small_square"  ; too::views::SliderStyle::small_square
        /// Medium track and large knob
        Large        = "large"         ; too::views::SliderStyle::large
        /// Large track and large knob
        LargeFilled  = "large_filled"  ; too::views::SliderStyle::large_filled
    }
}

make_style! {
    style SliderStyle is "SliderStyle" ; too::views::SliderStyle {
        /// The color of the track
        track_color   = Option<Color>  ; "Color?"
        /// The color of the knob
        knob_color    = Option<Color>  ; "Color?"
        /// The color of the track, when hovered
        track_hovered = Option<Color>  ; "Color?"
        /// The color of the knob, when hovered
        knob_hovered  = Option<Color>  ; "Color?"
        /// The character to use for the knob
        knob          = Option<String> ; "string?"
        /// The character to use for the track
        track         = Option<String> ; "string?"
    }
}

make_struct! {
    struct SliderParams is "SliderParams" {
        /// The style of the slider
        style = Option<SliderStyle> ; "SliderStyle?"
        /// The class of the slider
        class = Option<SliderClass> ; "Slider?"
        /// Axis to use for layout
        axis  = Option<Axis>        ; "Axis?"
        /// The value to use (an f32 in the range of 0.0 ..= 1.0)
        value = AnyUserData         ; "Value"
    }
}

impl Params<too::views::SliderStyle> for SliderParams {
    type Class = SliderClass;
    type Style = SliderStyle;

    fn class(&self) -> &Option<Self::Class> {
        &self.class
    }
    fn style(&self) -> &Option<Self::Style> {
        &self.style
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Slider;

impl BindingView for Slider {
    const SPEC: BindingSpec = binding! {
        /// A slider to adjust a value
        "slider" => "SliderParams"
    };
    type Params = SliderParams;
    type Style = SliderStyle;

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.foo::<SliderParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "slider", "params");
        };

        let Some(mut value) = ctx.value_mut(&params.value) else {
            return Mapping::report_missing_data(ui, ctx.id, "slider", "value");
        };

        let Some(value) = value.float_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "slider", "float");
        };

        let axis = params.axis.unwrap_or_default();
        let view = too::views::slider(value)
            .axis(axis.into())
            .class(params.apply_styling());
        ui.show(view);
    }
}
