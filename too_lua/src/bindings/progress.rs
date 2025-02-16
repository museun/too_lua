use too::view::{Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    params::{self, Value},
    Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Progress;

impl Progress {
    binding! {
        /// A progress bar
        "progress" => "Value | progress" {
            /// The style of the progress bar
            style "ProgressStyle?"
            /// The class of the progress bar
            class "Progress?"
            /// Axis to use for layout
            axis "Axis?"
            /// The value to use (an f32 in the range of 0.0 ..= 1.0)
            value "Value"
        }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let params = ctx.params::<params::ProgressParams>();
        let axis = ctx.axis();

        let Some(value) = ctx.value() else {
            return Mapping::report_missing(ui, ctx.id, "progress");
        };
        let Value::Float(value) = *value else {
            return Mapping::report_missing(ui, ctx.id, "float value");
        };

        let mut view = too::views::progress(value).axis(axis);
        if let Ok(params) = params {
            view = view.class(params.apply())
        }
        ui.show(view);
    }
}
