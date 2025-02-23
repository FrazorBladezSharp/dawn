use crate::applicationinfo::*;
use sfml::graphics::{RectangleShape, RenderTarget};
use specs::{Join, WorldExt};
use super::{ecs::components::*, App};

pub fn render_all(main_app: &mut App) {
    // clear the back buffer
    main_app.window.clear(BACKGROUND_COLOR);

    // draw all current objects to the back buffer
    draw_rectangles(main_app);

    // swap the completed back buffer to the front
    main_app.window.display();
}

fn draw_rectangles(app: &mut App) {
    // Gather the data
    let rectangles = app.world.read_storage::<CRectangle>();

    // go through each set of data
    for rect in (&rectangles).join() {
        // render
        app.window.draw(&RectangleShape::from_rect(rect.rectangle));
    }
}
