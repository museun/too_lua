use crate::make_enum;

make_enum! {
    CrossAlign {
        Start   = "min"
        End     = "max"
        Center  = "center"
        Stretch = "stretch"
        Fill    = "fill"
    }
}
