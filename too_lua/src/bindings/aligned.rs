use anno_lua::Anno;
use too::view::Ui;

use crate::{Context, Mapping, None, Spec, View};

#[derive(Copy, Clone, Debug, PartialEq, Anno, serde::Deserialize)]
#[anno(name = "Aligned", self)]
pub enum AlignedKind {
    /// Align to the horizontal left and vertical top
    #[anno(name = "left_top")]
    #[serde(rename = "left_top")]
    LeftTop,

    /// Align to the horizontal center and vertical top
    #[anno(name = "center_top")]
    #[serde(rename = "center_top")]
    CenterTop,

    /// Align to the horizontal right and vertical top
    #[anno(name = "right_top")]
    #[serde(rename = "right_top")]
    RightTop,

    /// Align to the horizontal left and vertical center
    #[anno(name = "left_center")]
    #[serde(rename = "left_center")]
    LeftCenter,

    /// Align to the horizontal center and vertical center
    #[anno(name = "center")]
    #[serde(rename = "center")]
    CenterCenter,

    /// Align to the horizontal right and vertical center
    #[anno(name = "right_center")]
    #[serde(rename = "right_center")]
    RightCenter,

    /// Align to the horizontal left and vertical bottom
    #[anno(name = "left_bottom")]
    #[serde(rename = "left_bottom")]
    LeftBottom,

    /// Align to the horizontal center and vertical bottom
    #[anno(name = "center_bottom")]
    #[serde(rename = "center_bottom")]
    CenterBottom,

    /// Align to the horizontal right and vertical bottom
    #[anno(name = "right_bottom")]
    #[serde(rename = "right_bottom")]
    RightBottom,
}

register_enum! {
    AlignedKind is "Aligned"
}

impl From<AlignedKind> for too::layout::Align2 {
    fn from(value: AlignedKind) -> Self {
        match value {
            AlignedKind::LeftTop => Self::LEFT_TOP,
            AlignedKind::CenterTop => Self::CENTER_TOP,
            AlignedKind::RightTop => Self::RIGHT_TOP,
            AlignedKind::LeftCenter => Self::LEFT_CENTER,
            AlignedKind::CenterCenter => Self::CENTER_CENTER,
            AlignedKind::RightCenter => Self::RIGHT_CENTER,
            AlignedKind::LeftBottom => Self::LEFT_BOTTOM,
            AlignedKind::CenterBottom => Self::CENTER_BOTTOM,
            AlignedKind::RightBottom => Self::RIGHT_BOTTOM,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Anno, serde::Deserialize)]
/// Parameter for `ui.aligned`
#[anno(exact)]
pub struct AlignedParams {
    /// Alignment for its children
    #[anno(lua_type = "Aligned")]
    pub align: AlignedKind,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Aligned;

impl View for Aligned {
    type Params = AlignedParams;
    type Style = None;

    fn spec() -> Spec {
        view_spec! {
            /// Align its children at a specific anchor
            Self {
                name: "aligned",
                params: "AlignedParams"
            }
        }
    }

    fn view(mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(params) = ctx.params_de::<AlignedParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "aligned", "params");
        };
        ui.aligned(params.align.into(), |ui| ctx.visit_children(mapping, ui));
    }
}
