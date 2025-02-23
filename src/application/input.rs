mod keyboard;
mod mouse;

use super::App;
use sfml::window::{Event, Key};

pub fn input_events(main_app: &mut App) -> bool {
    while let Some(event) = main_app.window.poll_event() {
        match event {
            // events from the window controller (OS)
            // false = App should close
            Event::Closed => return false,

            // App flow control events from the keyboard
            // false = App should close
            Event::KeyPressed {
                code: Key::Escape, ..
            } => return false,

            // default: ignore other events
            _ => {},
        }
    }
    // respond to user keyboard input
    keyboard::keyboard_events(main_app);
    
    // return to App.is_running
    true
}
