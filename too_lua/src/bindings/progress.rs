use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

use super::{Color, Value};

crate::make_proxy! {
    ProgressParams {
        class:
        ProgressClass is "Progress" {
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

        style:
        ProgressStyle => too::views::ProgressStyle {
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
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Progress;

impl Progress {
    binding! {
        /// A progress bar
        "progress" => "Value | number | progress" {
            /// The style of the progress bar
            style "ProgressStyle?"
            /// The class of the progress bar
            class "Progress?"
            /// Axis to use for layout
            axis "Axis?"
            /// The value to use (an f32 in the range of 0.0 ..= 1.0)
            value "Value | number"
        }
    }

    pub fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let params = ctx.params::<ProgressParams>();
        let axis = ctx.axis();

        let value = match ctx.value().and_then(|value| {
            let Value::Float(value) = *value else {
                return None;
            };
            Some(value)
        }) {
            Some(value) => value,
            None => {
                let Some(value) = ctx.param_value::<mlua::Number>() else {
                    return Mapping::report_missing_data(
                        ui,
                        ctx.id,
                        "progress",
                        &format!("float value: {:?}", ctx.current.data,),
                    );
                };
                value as f32
            }
        };

        let mut view = too::views::progress(value).axis(axis);
        if let Ok(params) = params {
            view = view.class(params.apply())
        }
        ui.show(view);
    }
}
