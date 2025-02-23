mod ball;
mod bat;

use super::components::*;
use crate::application::App;
use specs::{Join, WorldExt};

pub fn update(app: &App) {
    // variables
    // TODO: gather data using a query system ???
    // : eg let (rectangles , velocities) = query_get_storage(CRectangle, CVelocity); ??
    let names = app.world.read_storage::<CName>();
    let mut rectangles = app.world.write_storage::<CRectangle>();
    let mut velocities = app.world.write_storage::<CVelocity>();

    // update general systems
    bat::controll_bat(
        app,
        String::from("Bat"),
        &names,
        &mut rectangles,
        &mut velocities,
    );
    move_all(app, &mut rectangles, &mut velocities);

    ball::detect_window_boarder(
        String::from("Ball"),
        &names,
        &mut rectangles,
        &mut velocities,
    );
    
    ball::ball_bat_collision(&names, &mut rectangles, &mut velocities);
}

fn move_all(
    app: &App,
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
    for (rect, velocity) in (rectangles, velocities).join() {
        // increase the position by the velocity * app.delta_time
        rect.rectangle.left += velocity.x * app.delta_time;
        rect.rectangle.top += velocity.y * app.delta_time;
    }
}
