use crate::make_enum;

make_enum! {
    enum Aligned is "Aligned" {
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
