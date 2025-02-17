pub mod component;
pub mod entities;
pub mod systems;

use component::*;
use specs::{World, WorldExt};

pub fn register_components(app_world: &mut World) {
    // register each Component to the ecs world storage
    app_world.register::<CPosition>();
    app_world.register::<CVelocity>();
    app_world.register::<CSize>();
}
