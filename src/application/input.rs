use sfml::window::{Event, Key};
use super::App;

pub fn input_events(main_app: &mut App) /*-> result */{

    while let Some(event) = main_app.window.poll_event() {
        match event{
            // TODO: this file should never close the window !!!
            
            // events from the window controller (OS)
            Event::Closed => main_app.window.close(),

            // events from the keyboard
            Event::KeyPressed {code: Key::Escape, ..} => main_app.window.close(),

            // events from the mouse

            // default: we ignore the event
            _ => {}
        }
    }
}
