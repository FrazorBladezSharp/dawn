mod ecs;
mod input;
mod render;
mod window_api;

use crate::applicationinfo::*;
use ecs::*;
use sfml::cpp::FBox;
use sfml::graphics::RenderWindow;
use specs::{World, WorldExt};

// Holds the main loop for the application

// A structure to hold all important data for use in the application
pub struct App {
    window: FBox<RenderWindow>,
    is_running: bool,
    world: World,
}

// implimentation of main application
impl App {
    // create and initialise
    pub fn new() -> Self {
        println!("A new application is being created");

        // create a window
        let app_window =
            window_api::create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, MAX_FPS);

        let mut ecs_world = World::new();
        register_components(&mut ecs_world);

        // create our game entities
        entities::create_entities(&mut ecs_world);

        App {
            window: app_window,
            is_running: true,
            world: ecs_world,
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
