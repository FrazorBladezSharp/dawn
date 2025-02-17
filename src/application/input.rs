use sfml::window::{Event, Key};
use super::App;

pub fn input_events(main_app: &mut App) -> bool{

    while let Some(event) = main_app.window.poll_event() {
        match event{            
            // events from the window controller (OS)
            Event::Closed => return false,

            // events from the keyboard
            Event::KeyPressed {code: Key::Escape, ..} => return false,

            // events from the mouse

            // default: we ignore the event
            _ => {}
        }
    }

    true
}
