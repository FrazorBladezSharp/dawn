//use sfml::graphics::Rect;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Default)]
pub struct CPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
pub struct CVelocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
pub struct CSize {
    pub width: f32,
    pub height: f32,
}
/*
#[derive(Component, Default)]
struct CRectangle{
    rectangle: Rect<f32>,
}

fn make_rectangle(){
    let base_rect = CRectangle::default();

    let thing = RectangleShape::from_rect(base_rect.rectangle);
}
*/
