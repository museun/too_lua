use too::view::Ui;

use crate::{
    mapping::{BindingSpec, BindingView},
    Context, Mapping,
};

crate::make_enum! {
    enum AlignedKind is "Aligned" {
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

crate::make_struct! {
    struct AlignParams is "AlignParams" {
        /// Alignment for its children
        align = AlignedKind ; "Aligned"
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Aligned;

impl BindingView for Aligned {
    const SPEC: BindingSpec = binding! {
        /// Align its children at a specific anchor
        "aligned" => "AlignParams"
    };

    type Params = AlignParams;
    type Style = ();

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        use too::layout::Align2;

        let Some(params) = ctx.foo::<AlignParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "aligned", "params");
        };

        let align = match params.align {
            AlignedKind::LeftTop => Align2::LEFT_TOP,
            AlignedKind::CenterTop => Align2::CENTER_TOP,
            AlignedKind::RightTop => Align2::RIGHT_TOP,
            AlignedKind::LeftCenter => Align2::LEFT_CENTER,
            AlignedKind::CenterCenter => Align2::CENTER_CENTER,
            AlignedKind::RightCenter => Align2::RIGHT_CENTER,
            AlignedKind::LeftBottom => Align2::LEFT_BOTTOM,
            AlignedKind::CenterBottom => Align2::CENTER_BOTTOM,
            AlignedKind::RightBottom => Align2::RIGHT_BOTTOM,
        };

        ui.aligned(align, |ui| ctx.visit_children(mapping, ui));
    }
}
