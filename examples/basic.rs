use gtk::prelude::*;
use alacritty_gobject::Terminal;

fn main() {
    // Create a new application
    let app = gtk::Application::new(Some("com.DefunctLizard.example"), Default::default());
    
    app.connect_activate(|app| {
        on_activate(app);
    });

    // Run the application
    app.run();
}

// When the application is launched…
fn on_activate(application: &gtk::Application) {
    // … create a new window …
    let window =gtk::ApplicationWindow::new(application);
    window.set_title(Some("My GTK App"));

    // Create a button
    let button = Terminal::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // Add button
    window.set_child(Some(&button));
    window.present();
}
