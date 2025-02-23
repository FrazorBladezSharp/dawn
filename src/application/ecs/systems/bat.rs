use crate::{
    application::{ecs::components::*, App},
    applicationinfo::*,
};
use specs::Join;

pub fn controll_bat(
    app: &App,
    bat_object: String,
    names: &specs::Storage<
        '_,
        CName,
        specs::shred::Fetch<'_, specs::storage::MaskedStorage<CName>>,
    >,
    rectangles: &mut specs::Storage<
        '_,
        CRectangle,
        specs::shred::FetchMut<'_, specs::storage::MaskedStorage<CRectangle>>,
    >,
    velocities: &mut specs::Storage<
        '_,
        CVelocity,
        specs::shred::FetchMut<'_, specs::storage::MaskedStorage<CVelocity>>,
    >,
) {
    for (name, rect, vel) in (names, rectangles, velocities).join() {
        if name.name == bat_object {
            // set the velocity relative to the direction of movement
            vel.x = (app.bat_move as i32 - 2) as f32 * BAT_SPEED;

            // stop the bat going passed the edge of the screen
            if (rect.rectangle.left <= 0.0 && app.bat_move == MoveDirection::Left)
                || (rect.rectangle.left + rect.rectangle.width >= WINDOW_WIDTH as f32
                    && app.bat_move == MoveDirection::Right)
            {
                vel.x = 0.0;
            }
        }
    }
}
