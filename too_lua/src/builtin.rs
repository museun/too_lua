use anno_lua::Anno;

use crate::{
    Proxy,
    bindings::Color,
    runtime::{Duration, Runtime},
};

#[derive(Copy, Clone, Debug, Anno, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[anno(name = "Palette", alias = "Color")]
pub enum PaletteKind {
    /// The background color
    #[anno(name = "background")]
    Background,
    /// The foreground color
    #[anno(name = "foreground")]
    Foreground,
    /// A color close to the background, but more visible
    #[anno(name = "surface")]
    Surface,
    /// A color used to outline things.
    ///
    /// This is generally like surface, but even more visible
    #[anno(name = "outline")]
    Outline,
    /// A color used to contrast something against the background
    #[anno(name = "contrast")]
    Contrast,
    /// A color used for a primary action
    ///
    /// e.g. the default interaction color
    #[anno(name = "primary")]
    Primary,
    /// A color used for a secondary action
    ///
    /// e.g an interaction color that is different from the primary color
    #[anno(name = "secondary")]
    Secondary,
    /// A accent color used to differentiate something from a primary and secondary color
    #[anno(name = "accent")]
    Accent,
    /// A color representing that something is dangerous
    #[anno(name = "danger")]
    Danger,
    /// A color representing that something is successful
    #[anno(name = "success")]
    Success,
    /// A color representing that something is potentially dangerous
    #[anno(name = "warning")]
    Warning,
    /// A color representing that something should be noted
    #[anno(name = "info")]
    Info,
}

register_enum! {
    PaletteKind is "Palette"
}

pub const BUILTIN: &[Proxy] = &[
    Proxy::new::<Runtime>(), //
    Proxy::new::<Duration>(),
    Proxy::new::<PaletteKind>(),
    Proxy::new::<Color>(),
];
