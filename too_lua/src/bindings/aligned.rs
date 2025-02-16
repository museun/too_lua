use too::view::Ui;

use crate::{
    mapping::{Binding, Field},
    params, Context, Mapping,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Aligned;

impl Aligned {
    binding! {
        /// Align its children at a specific anchor
        "aligned" => "aligned" {
            /// Alignment for the children
            align "Aligned"
        }
    }

    pub fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(Ok(aligned)) = ctx.params_field::<params::Aligned>("align") else {
            return Mapping::report_missing(ui, ctx.id, "aligned");
        };

        use too::layout::Align2;
        let align = match aligned {
            params::Aligned::LeftTop => Align2::LEFT_TOP,
            params::Aligned::CenterTop => Align2::CENTER_TOP,
            params::Aligned::RightTop => Align2::RIGHT_TOP,
            params::Aligned::LeftCenter => Align2::LEFT_CENTER,
            params::Aligned::CenterCenter => Align2::CENTER_CENTER,
            params::Aligned::RightCenter => Align2::RIGHT_CENTER,
            params::Aligned::LeftBottom => Align2::LEFT_BOTTOM,
            params::Aligned::CenterBottom => Align2::CENTER_BOTTOM,
            params::Aligned::RightBottom => Align2::RIGHT_BOTTOM,
        };

        ui.aligned(align, |ui| ctx.visit_children(mapping, ui));
    }
}
