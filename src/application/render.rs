use super::{ecs::components::*, App};
use sfml::graphics::{Color, RectangleShape, RenderTarget};
use specs::{Join, WorldExt};

// values used are between 0 and 255
const BACKGROUND_COLOR: Color = Color {
    r: 10,
    g: 10,
    b: 55,
    a: 255,
};

pub fn render_all(main_app: &mut App) {
    // create a list of all things to be redered

    // clear the back buffer
    main_app.window.clear(BACKGROUND_COLOR);

    // draw all current objects to the back buffer
    // main_app.window.draw( Object to draw );
    draw_rectangles(main_app);

    // swap the completed back buffer to the front
    main_app.window.display();
}

// TODO: why are we constructing the rectangle here ???
//  : can this be done when we create the entity ?
fn draw_rectangles(app: &mut App) {
    // Gather the data
    let rectangles = app.world.read_storage::<CRectangle>();

    // go through each set of data
    for rect in (&rectangles).join() {
        // render
        app.window.draw(&RectangleShape::from_rect(rect.rectangle));
    }
}
