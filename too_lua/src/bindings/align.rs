use anno_lua::Anno;

#[derive(Copy, Clone, Debug, Default, PartialEq, Anno, serde::Deserialize)]
#[anno(self)]
pub enum Align {
    /// Align to the start of the area
    #[anno(name = "min")]
    #[serde(rename = "min")]
    #[default]
    Min,

    /// Align to the middle of the area
    #[anno(name = "middle")]
    #[serde(rename = "middle")]
    Middle,

    /// Align to the end of the area
    #[anno(name = "max")]
    #[serde(rename = "max")]
    Max,
}

register_enum! {
    Align is "Align"
}

impl From<Align> for too::layout::Align {
    fn from(value: Align) -> Self {
        match value {
            Align::Min => Self::Min,
            Align::Middle => Self::Center,
            Align::Max => Self::Max,
        }
    }
}
