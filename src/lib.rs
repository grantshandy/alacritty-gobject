// Special thanks to https://github.com/unrelentingtech/galacritty/ for inspiration.
use std::default::Default;
// use std::sync::Arc;

// use glutin::event_loop::EventLoop as GlutinEventLoop;
pub mod window;
pub mod support;

use gtk::glib;
use gtk::prelude::*;

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
        type ParentType = gtk::GLArea;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for Terminal {}

    // Trait shared by all widgets
    impl WidgetImpl for Terminal {}

    // Trait for GLArea
    impl GLAreaImpl for Terminal {}
}

glib::wrapper! {
    pub struct Terminal(ObjectSubclass<imp::Terminal>)
        @extends gtk::GLArea, gtk::Widget;
}

impl Terminal {
    // Creates a new Terminal with default configurations.
    pub fn new() -> Self {
        let gl_area: Terminal = glib::Object::new(&[]).expect("Failed to create Terminal Widget");
    
        return gl_area;
    }
}
