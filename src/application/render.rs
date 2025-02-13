use sfml::graphics::{Color, RenderTarget};

use super::App;

const BACKGROUND_COLOR: Color = Color {
    r: 10,
    g: 10,
    b: 55,
    a: 255,
};

pub fn render_all(main_app: &mut App) {
    // clear the back buffer 
    main_app.window.clear(BACKGROUND_COLOR);

    //window.draw( Object to draw );

    // swap the completed back buffer to the front
    main_app.window.display();
}
