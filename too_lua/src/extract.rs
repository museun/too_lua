use std::borrow::Cow;

use too::renderer::Rgba;

use crate::bindings::Color;

// TODO maybe support mlua::String
// this'll involve Extrat being generic over R rather than as an associated type
pub trait Extract {
    type R;
    fn extract(&mut self, right: &Self::R);
}

impl Extract for char {
    type R = Option<String>;
    fn extract(&mut self, right: &Self::R) {
        if let Some(right) = right {
            *self = right.chars().nth(0).unwrap_or(' ')
        }
    }
}

impl Extract for bool {
    type R = Option<bool>;
    fn extract(&mut self, right: &Self::R) {
        if let Some(right) = right.filter(|c| *c) {
            *self = right
        }
    }
}

impl Extract for Cow<'static, str> {
    type R = Option<String>;
    fn extract(&mut self, right: &Self::R) {
        if let Some(right) = right {
            *self = right.clone().into()
        }
    }
}

impl Extract for Rgba {
    type R = Option<Color>;
    fn extract(&mut self, right: &Self::R) {
        if let Some(right) = right {
            *self = right.0
        }
    }
}

impl Extract for Option<Rgba> {
    type R = Option<Color>;
    fn extract(&mut self, right: &Self::R) {
        if let Some(right) = right {
            *self = Some(right.0)
        }
    }
}

pub fn merge<E>(left: &mut dyn Extract<R = E>, right: &E) {
    left.extract(right);
}
