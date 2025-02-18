use sfml::graphics::Rect;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Default)]
struct CRectangle {
    rectangle: Rect<f32>,
}

#[derive(Component, Default)]
pub struct CVelocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
pub struct CPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
pub struct CSize {
    pub width: f32,
    pub height: f32,
}


/* ideas on how to use the sfml Rect as a component.

    .with(CRectangle { rectangle: Rect::new(0.0, 0.0, 1.0, 1.0) });

    let drawable = RectangleShape::from_rect(base_rect.rectangle);


fn test() {
    let x = CRectangle {
        rectangle: Rect::new(0.0, 0.0, 1.0, 1.0),
    };
}
*/
