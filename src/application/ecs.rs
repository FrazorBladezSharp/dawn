
pub mod create_entities;
pub mod component;
pub mod systems;

use component::*;
use specs::{World, WorldExt};


pub fn register_components(app_world: &mut World){
    app_world.register::<Position>();
    //app_world.register::<Velocity>();
    app_world.register::<Size>();
}
