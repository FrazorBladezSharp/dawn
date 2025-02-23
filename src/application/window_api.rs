use sfml::{cpp::FBox, graphics::RenderWindow, window::Style};

// create a window with some default settings
pub fn create_window(width: u32, height: u32, title: &str, max_fps: u32) -> FBox<RenderWindow> {
    // use the SFML render window for graphical output rendering
    let mut win = RenderWindow::new((width, height), title, Style::CLOSE, &Default::default())
        .expect("Error: failed to open window");

    // give a limit to how fast we update the window
    // this will avoid graphical issues such as tearing
    win.set_framerate_limit(max_fps);

    // return the window handle to the user
    win
}
