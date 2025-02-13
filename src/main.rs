mod application;
mod applicationinfo;

use application::App;

// main entry point, initialise and run the application

// Goals: to create a generic application foundation

fn main() {
    println!("\n Welcome to the Application Sandbox\n");

    // Create an initialised application
    let mut application = App::new();

    // Run the applications main loop
    application.run();

    // exit the application
    println!("The Application has shutdown");
}
