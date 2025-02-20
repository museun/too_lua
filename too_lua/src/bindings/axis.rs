use anno_lua::Anno;

#[derive(Copy, Clone, Debug, Default, PartialEq, Anno)]
#[anno(self)]
pub enum Axis {
    /// The vertical axis
    #[anno(name = "vertical")]
    Vertical,

    /// The horizontal axis
    #[anno(name = "horizontal")]
    #[default]
    Horizontal,
}

register_enum! {
    Axis is "Axis"
}

impl From<Axis> for too::layout::Axis {
    fn from(value: Axis) -> Self {
        match value {
            Axis::Vertical => Self::Vertical,
            Axis::Horizontal => Self::Horizontal,
        }
    }
}
