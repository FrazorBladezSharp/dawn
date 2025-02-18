use sfml::graphics::Rect;
use specs::{Builder, World, WorldExt};
use super::components::*;

// TODO: are there better ways to do this ???
//  : use an initial data set (array) and loop ?
//  : read data in from an external file ??

const BAT_DATA: [f32; 6] = [500.0, 500.0, 100.0, 20.0, 1.0, 1.0];
const BALL_DATA: [f32; 6] = [525.0, 250.0, 20.0, 20.0, 1.0, 1.0];

pub fn create_entities(app_world: &mut World){
    // call init with each entities data array
    init(app_world, BAT_DATA);
    init(app_world, BALL_DATA);
}

fn init(app_world: &mut World, data: [f32; 6]){
    app_world
        .create_entity()
        .with(CRectangle { rectangle: Rect::new(data[0], data[1], data[2], data[3]) })
        .with(CVelocity { x: data[4], y: data[5] })
        .build();
}
