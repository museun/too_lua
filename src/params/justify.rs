use crate::make_enum;

make_enum! {
    Justify {
        Start        = "min"
        End          = "max"
        Center       = "center"
        SpaceBetween = "space_between"
        SpaceAround  = "space_around"
        SpaceEvenly  = "space_evenly"
    }
}
