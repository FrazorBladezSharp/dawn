mod input;
mod render;
mod window_api;

use crate::applicationinfo::*;
use sfml::cpp::FBox; 
use sfml::graphics::RenderWindow;

// Holds the main loop for the application

// A structure to hold all important data for use in the application
pub struct App {
    window: FBox<RenderWindow>,
    is_running: bool,
}

// implimentation of main application
impl App {
    // create and initialise
    pub fn new() -> Self {
        println!("A new application is being created");

        // create a window
        let app_window =
            window_api::create_window(
                WINDOW_WIDTH, 
                WINDOW_HEIGHT, 
                WINDOW_TITLE, 
                MAX_FPS
            );

        App { 
            window: app_window,
            is_running: true, 
        }
    }

    // run the main loop
    pub fn run(&mut self) {
        println!("Application is running");

        // main application loop
        while self.is_running {
            // poll the OS events
            self.is_running = input::input_events(self);

            // update our application (ECS ?, GUI ?)

            // render to window
            render::render_all(self);
        }

        // shutdown
        self.window.close();
    }
}
