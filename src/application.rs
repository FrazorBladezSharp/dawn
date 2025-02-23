mod ecs;
mod input;
mod render;
mod window_api;
mod dusk_api;

use crate::applicationinfo::*;
use ecs::*;
use sfml::graphics::RenderWindow;
use sfml::system::Clock;
use sfml::{cpp::FBox, system::Time};
use specs::{World, WorldExt};

// constructs the application and its data
//  : Holds the main loop for the application

// A structure to hold all important data for use in the application
pub struct App {
    window: FBox<RenderWindow>,
    world: World,
    delta_time: f32,
    is_running: bool,
    bat_move: MoveDirection,
}

// implimentation of the main application
impl App {
    // create and initialise
    pub fn new() -> Self {
        println!("A new application is being created");

        // create a window
        let app_window =
            window_api::create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, MAX_FPS);

        // create the ecs storage
        let mut ecs_world = World::new();

        // store all components into the ecs
        register_components(&mut ecs_world);

        // create our game entities and store in ecs
        entities::create_entities(&mut ecs_world);

        // return the completed App to the user
        // TODO: replace is_running with an expandable state machine
        App {
            window: app_window,
            world: ecs_world,
            delta_time: Time::ZERO.as_seconds(),
            is_running: true,
            bat_move: MoveDirection::Stationary,
        }
    }

    // run the main application loop
    pub fn run(&mut self) {
        println!("Application is running");

        // Create a clock to measure delta time
        let mut clock = Clock::start().expect("error: unable to create a dt clock");

        // main application loop
        while self.is_running {
            // poll all input and OS events
            self.is_running = input::input_events(self);

            // Calculate delta time
            self.delta_time = clock.restart().as_seconds();

            // stop wierd behavoir if we poll events for too long
            if self.delta_time <= 0.02 {
                // update our application (ECS, GUI ?)
                ecs::systems::update(self);

                // render to the applications window
                render::render_all(self);
            }
        }

        // shutdown with a clean exit to the OS
        self.window.close();
    }
}
