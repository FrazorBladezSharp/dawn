use super::component::*;
use specs::{Builder, World, WorldExt};

// TODO: are there better ways to do this ???
//  : use an initial data set (array) and loop ?
//  : read data in from an external file ??

pub fn create_bat(app_world: &mut World) {
    app_world
        .create_entity()
        .with(CPosition { x: 500.0, y: 500.0 })
        .with(CVelocity { x: 1.0, y: 1.0 })
        .with(CSize {
            width: 50.0,
            height: 20.0,
        })
        .build();
}

pub fn create_ball(app_world: &mut World) {
    app_world
        .create_entity()
        .with(CPosition { x: 525.0, y: 250.0 })
        .with(CVelocity { x: 1.0, y: 1.0 })
        .with(CSize {
            width: 10.0,
            height: 10.0,
        })
        .build();
}
