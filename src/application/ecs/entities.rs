use super::components::*;
use sfml::graphics::Rect;
use specs::{Builder, World, WorldExt};

// TODO: are there better ways to do this ???
//  : use an initial data set (array) and loop ?
//  : read data in from an external file ??

pub fn create_bat(app_world: &mut World) {
    app_world
        .create_entity()
        .with(CRectangle { rectangle: Rect::new(500.0, 500.0, 100.0, 20.0) })
        .with(CVelocity { x: 1.0, y: 1.0 })
        .build();
}

pub fn create_ball(app_world: &mut World) {
    app_world
        .create_entity()
        .with(CRectangle { rectangle: Rect::new(525.0, 250.0, 20.0, 20.0) })
        .with(CVelocity { x: 1.0, y: 1.0 })
        .build();
}
