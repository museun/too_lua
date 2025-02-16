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
