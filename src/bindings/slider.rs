use too::view::{Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    params::{self, Value},
    Context, Mapping,
};

pub struct Slider;
impl Slider {
    binding! {
        /// A slider to adjust a value
        "slider" => "Value | slider" {
            /// The style of the slider
            style "SliderStyle?"
            /// The class of the slider
            class "Slider?"
            /// Axis to use for layout
            axis "Axis?"
            /// The value to use (an f32 in the range of 0.0 ..= 1.0)
            value "Value"
        }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let params = ctx.params::<params::SliderParams>();
        let axis = ctx.axis();

        let Some(mut v) = ctx.value_mut() else {
            return Mapping::report_missing(ui, ctx.id, "slider");
        };
        let Value::Float(v) = &mut *v else {
            return Mapping::report_missing(ui, ctx.id, "float value");
        };

        let mut view = too::views::slider(v).axis(axis);
        if let Ok(params) = params {
            view = view.class(params.apply())
        }
        ui.show(view);
    }
}
