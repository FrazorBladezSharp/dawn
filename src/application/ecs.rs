pub mod components;
pub mod entities;
pub mod systems;

use components::*;
use specs::{World, WorldExt};

pub fn register_components(app_world: &mut World) {
    // register each Component to the ecs world storage
    app_world.register::<CName>();
    app_world.register::<CRectangle>();
    app_world.register::<CVelocity>();
}
