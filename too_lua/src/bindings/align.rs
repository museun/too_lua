use anno_lua::Anno;

#[derive(Copy, Clone, Debug, Default, PartialEq, Anno)]
#[anno(self)]
pub enum Align {
    /// Align to the start of the area
    #[anno(name = "min")]
    #[default]
    Min,

    /// Align to the center of the area
    #[anno(name = "center")]
    Center,

    /// Align to the end of the area
    #[anno(name = "max")]
    Max,
}

register_enum! {
    Align is "Align"
}

impl From<Align> for too::layout::Align {
    fn from(value: Align) -> Self {
        match value {
            Align::Min => Self::Min,
            Align::Center => Self::Center,
            Align::Max => Self::Max,
        }
    }
}
