use anno_lua::Anno;

#[derive(Copy, Clone, Debug, Default, PartialEq, Anno)]
#[anno(self)]
pub enum CrossAlign {
    /// Alignment begins at the 'start' of the area
    #[anno(name = "min")]
    #[default]
    Start,

    /// Alignment begins at the 'end' of the area
    #[anno(name = "max")]
    End,

    /// Alignment is in the middle, extra space is applied before and after
    #[anno(name = "center")]
    Center,

    /// The elements stretch to fill the area
    #[anno(name = "stretch")]
    Stretch,

    /// The elements fill the entire area
    #[anno(name = "fill")]
    Fill,
}

register_enum! {
    CrossAlign is "CrossAlign"
}

impl From<CrossAlign> for too::layout::CrossAlign {
    fn from(value: CrossAlign) -> Self {
        match value {
            CrossAlign::Start => Self::Start,
            CrossAlign::End => Self::End,
            CrossAlign::Center => Self::Center,
            CrossAlign::Stretch => Self::Stretch,
            CrossAlign::Fill => Self::Fill,
        }
    }
}
