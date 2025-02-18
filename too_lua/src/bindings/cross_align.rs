use crate::make_enum;

make_enum! {
    enum CrossAlign is "CrossAlign" {
        /// Alignment begins at the 'start' of the area
        Start   = "min"
        /// Alignment begins at the 'end' of the area
        End     = "max"
        /// Alignment is in the middle, extra space is applied before and after
        Center  = "center"
        /// The elements stretch to fill the area
        Stretch = "stretch"
        /// The elements fill the entire area
        Fill    = "fill"
    }
}

impl Default for CrossAlign {
    fn default() -> Self {
        Self::Start
    }
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
