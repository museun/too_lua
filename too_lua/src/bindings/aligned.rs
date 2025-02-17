use too::view::Ui;

use crate::{
    mapping::{Binding, Field},
    Context, Mapping,
};

crate::make_enum! {
    enum AlignedParams is "Aligned" {
        /// Align to the horizontal left and vertical top
        LeftTop      = "left_top"
        /// Align to the horizontal center and vertical top
        CenterTop    = "center_top"
        /// Align to the horizontal right and vertical top
        RightTop     = "right_top"
        /// Align to the horizontal left and vertical center
        LeftCenter   = "left_center"
        /// Align to the horizontal center and vertical center
        CenterCenter = "center"
        /// Align to the horizontal right and vertical center
        RightCenter  = "right_center"
        /// Align to the horizontal left and vertical bottom
        LeftBottom   = "left_bottom"
        /// Align to the horizontal center and vertical bottom
        CenterBottom = "center_bottom"
        /// Align to the horizontal right and vertical bottom
        RightBottom  = "right_bottom"
    }
}

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
        let Some(Ok(aligned)) = ctx.params_field::<AlignedParams>("align") else {
            return Mapping::report_missing(ui, ctx.id, "aligned");
        };

        use too::layout::Align2;
        let align = match aligned {
            AlignedParams::LeftTop => Align2::LEFT_TOP,
            AlignedParams::CenterTop => Align2::CENTER_TOP,
            AlignedParams::RightTop => Align2::RIGHT_TOP,
            AlignedParams::LeftCenter => Align2::LEFT_CENTER,
            AlignedParams::CenterCenter => Align2::CENTER_CENTER,
            AlignedParams::RightCenter => Align2::RIGHT_CENTER,
            AlignedParams::LeftBottom => Align2::LEFT_BOTTOM,
            AlignedParams::CenterBottom => Align2::CENTER_BOTTOM,
            AlignedParams::RightBottom => Align2::RIGHT_BOTTOM,
        };

        ui.aligned(align, |ui| ctx.visit_children(mapping, ui));
    }
}
