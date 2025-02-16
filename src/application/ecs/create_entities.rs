
use specs::{Builder, World, WorldExt};
use super::component::*;



pub fn create_bat(app_world: &mut World){
    app_world.create_entity()
        .with(Position{ x: 500.0, y: 500.0})
        //.with(Velocity::default())
        .with(Size{ width: 50.0, height: 20.0})
        .build();

}

pub fn create_ball(app_world: &mut World){
    app_world.create_entity()
        .with(Position{ x: 525.0, y: 250.0})
        //.with(Velocity::default())
        .with(Size{ width: 10.0, height: 10.0})
        .build();
}
