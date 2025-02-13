
use sfml::{cpp::FBox, graphics::RenderWindow, window::Style};

pub fn create_window(width: u32, height: u32, title: &str, max_fps: u32) -> FBox<RenderWindow> {
    let mut win = RenderWindow::new(
        (width, height), 
        title, 
        Style::DEFAULT, 
        &Default::default()
    ).expect("Error: failed to open window");

    win.set_framerate_limit(max_fps);

    win
}
