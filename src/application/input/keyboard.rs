use crate::{applicationinfo::MoveDirection, App};
use sfml::window::Key;

pub fn keyboard_events(main_app: &mut App) {
    //check key state for first key OR second key pressed
    let state = (Key::Left.is_pressed() as i32) + ((Key::Right.is_pressed() as i32) * 2);

    match state {
        1 => main_app.bat_move = MoveDirection::Left,
        2 => main_app.bat_move = MoveDirection::Right,
        _ => main_app.bat_move = MoveDirection::Stationary,
    }
}
