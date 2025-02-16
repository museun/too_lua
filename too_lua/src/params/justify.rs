use crate::make_enum;

make_enum! {
    enum Justify is "Justify" {
        /// The extra space is applied to the end
        Start        = "min"
        /// The extra space is applied to the start
        End          = "max"
        /// The extra space is applied to the start and end
        Center       = "center"
        /// Place the space between the elements
        SpaceBetween = "space_between"
        /// Place the space around the elements
        SpaceAround  = "space_around"
        /// Evenly space the elements
        SpaceEvenly  = "space_evenly"
    }
}
