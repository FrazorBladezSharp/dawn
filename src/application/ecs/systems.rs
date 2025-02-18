use crate::application::App;
use specs::{Join, WorldExt};
use super::components::*;

pub fn update(app: &mut App) {
    // update all our systems
    move_all(app);
}

fn move_all(app: &mut App) {
    // Gather data
    let mut rectangles = app.world.write_storage::<CRectangle>();
    let velocities = app.world.read_storage::<CVelocity>();

    // Go through each set of data pairs
    for (rect, velocity) in (&mut rectangles, &velocities).join() {
        // increase the position by the velocity * delta_time
        rect.rectangle.left = rect.rectangle.left + velocity.x * 1.0;
        rect.rectangle.top = rect.rectangle.top + velocity.y * 1.0;
    }
}
