use anno_lua::Anno;

#[derive(Copy, Clone, Debug, Default, PartialEq, Anno, serde::Deserialize)]
#[anno(self)]
pub enum Justify {
    /// The extra space is applied to the end
    #[anno(name = "min")]
    #[serde(rename = "min")]
    #[default]
    Start,

    /// The extra space is applied to the start
    #[anno(name = "max")]
    #[serde(rename = "max")]
    End,

    /// The extra space is applied to the start and end
    #[anno(name = "center")]
    #[serde(rename = "center")]
    Center,

    /// Place the space between the elements
    #[anno(name = "space_between")]
    #[serde(rename = "space_between")]
    SpaceBetween,

    /// Place the space around the elements
    #[anno(name = "space_around")]
    #[serde(rename = "space_around")]
    SpaceAround,

    /// Evenly space the elements
    #[anno(name = "space_evenly")]
    #[serde(rename = "space_evenly")]
    SpaceEvenly,
}

register_enum! {
    Justify is "Justify"
}

impl From<Justify> for too::layout::Justify {
    fn from(value: Justify) -> Self {
        match value {
            Justify::Start => Self::Start,
            Justify::End => Self::End,
            Justify::Center => Self::Center,
            Justify::SpaceBetween => Self::SpaceBetween,
            Justify::SpaceAround => Self::SpaceAround,
            Justify::SpaceEvenly => Self::SpaceEvenly,
        }
    }
}
