use sfml::graphics::Rect;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Default)]
pub struct CRectangle {
    pub rectangle: Rect<f32>,
}

#[derive(Component, Default)]
pub struct CVelocity {
    pub x: f32,
    pub y: f32,
}
