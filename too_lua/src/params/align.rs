use crate::make_enum;

make_enum! {
    enum Align is "Align" {
        /// Align to the start of the area
        Min    = "min"
        /// Align to the middle of the area
        Middle = "middle"
        /// Align to the end of the area
        Max    = "max"
    }
}
