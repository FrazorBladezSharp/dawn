// We can place public enums, types etc. here for access by the application

// constants ***********************************************************

use sfml::graphics::Color;

pub const WINDOW_TITLE: &str = "Application";
pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;

pub const MAX_FPS: u32 = 60;

// the window background color
// values used are between 0 and 255
pub const BACKGROUND_COLOR: Color = Color {
    r: 10,
    g: 10,
    b: 55,
    a: 255,
};

// TODO: read data in from an external file ??
pub const BAT_DATA: [f32; 6] = [500.0, 500.0, 100.0, 20.0, 0.0, 0.0];
pub const BALL_DATA: [f32; 6] = [125.0, 50.0, 20.0, 20.0, 200.0, 150.0];
pub const BAT_SPEED: f32 = 200.0;

// enums ***************************************************************

#[derive(Clone, Copy, PartialEq)]
pub enum MoveDirection {
    Left = 1,
    Stationary = 2,
    Right = 3,
}

// TODO: An enum to describe the available application states
//  : We can use a vec as a stack for holding the application's
//  : current and previous states
