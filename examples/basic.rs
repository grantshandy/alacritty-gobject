use alacritty_gobject::{Terminal, Program};
use gtk::prelude::*;

fn main() {
    // Create a new application
    let application = gtk::Application::new(Some("com.DefunctLizard.example"), Default::default());

    application.connect_activate(|application| {
        // Create a new window
        let window = gtk::ApplicationWindow::new(application);
        window.set_title(Some("alacritty-gobject example window"));

        // Create params for our terminal
        let program = Program::Just(String::from("/bin/bash"));
        let path = PathBuf::from("/");

        // Create a terminal
        let term = Terminal::new(program, Some(path));

        // Add our terminal to the window
        window.set_child(Some(&term));

        // Present the window
        window.present();
    });

    // Run the application
    application.run();
}
