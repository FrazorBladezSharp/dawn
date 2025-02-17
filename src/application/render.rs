use super::{ecs::component::*, App};
use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, Transformable},
    system::Vector2f,
};
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
    // varables
    let mut rectangle = RectangleShape::new();

    // Gather the data 
    let positions = app.world.read_storage::<CPosition>();
    let sizes = app.world.read_storage::<CSize>();

    // go through each set of data
    for (pos, siz) in (&positions, &sizes).join() {
        // create the rectangle to be drawn
        rectangle.set_position(Vector2f::new(pos.x, pos.y));
        rectangle.set_size(Vector2f::new(siz.width, siz.height));

        // render
        app.window.draw(&rectangle);
    }
}
