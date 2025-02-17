mod application;
mod applicationinfo;

use application::App;
use dusk::welcome;

// main entry point, initialise and run the application

// Goals: to create a generic application foundation

fn main() {
    // Welcome message from our Library
    welcome::welcome();

    // Create an initialised application
    let mut application = App::new();

    // Run the applications main loop
    application.run();

    // exit the application
    println!("The Application has shutdown\n");
}
