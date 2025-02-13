mod window_api;

use sfml::{cpp::FBox, graphics::RenderWindow, window::Event};
use crate::applicationinfo::*;

//use crate::applicationinfo;

// Holds the main loop for the application

// A structure to hold all important data for use in the application
pub struct App {
    window: FBox<RenderWindow>,
}

// implimentation of main application
impl App {
    // create and initialise
    pub fn new() -> Self {
        println!("A new application is being created");
        
        // create a window
        let app_window = window_api::create_window(
            WINDOW_WIDTH, 
            WINDOW_HIEGHT,
            WINDOW_TITLE, 
            MAX_FPS,
        );

        App {
            window: app_window,
        }
    }

    // run the main loop
    pub fn run(&mut self) {
        println!("Application is running");

        // main application loop
        while self.window.is_open(){
            // poll the OS events
            while let Some(event) = self.window.poll_event() {
                match event{
                    Event::Closed => self.window.close(),

                    _ => {}
                }
            }
            // update our application

            // render to window
        }

        // shutdown 
    }
}
