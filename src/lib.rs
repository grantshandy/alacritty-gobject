use gtk::glib;
use gtk::prelude::*;
use alacritty::display::Display;

// Implementation of our custom GObject
mod imp {
    // Import parent scope
    use super::*;
    // Import necessary traits for subclassing
    use gtk::subclass::prelude::*;

    // Object holding the state
    #[derive(Default)]
    pub struct Terminal;

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for Terminal {
        const NAME: &'static str = "Terminal";
        type Type = super::Terminal;
        type ParentType = gtk::Button;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for Terminal {}

    // Trait shared by all widgets
    impl WidgetImpl for Terminal {}

    // Trait shared by all buttons
    impl ButtonImpl for Terminal {}
}

glib::wrapper! {
    pub struct Terminal(ObjectSubclass<imp::Terminal>)
        @extends gtk::Button, gtk::Widget;
}

impl Terminal {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Button")
    }

    pub fn with_label(label: &str) -> Self {
        let button = Self::new();
        button.set_label(label);
        button
    }
}

pub fn start_term_window() {

}