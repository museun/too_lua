use crate::make_enum;

make_enum! {
    enum Axis is "Axis" {
        /// The vertical axis
        Vertical   = "vertical"
        /// The horizontal axis
        Horizontal = "horizontal"
    }
}

impl Default for Axis {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl From<Axis> for too::layout::Axis {
    fn from(value: Axis) -> Self {
        match value {
            Axis::Vertical => Self::Vertical,
            Axis::Horizontal => Self::Horizontal,
        }
    }
}
