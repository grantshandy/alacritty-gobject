// Special thanks to https://github.com/unrelentingtech/galacritty/ for inspiration
// use std::cell::RefCell;
// use std::os::unix::io::{RawFd, AsRawFd};
// use std::sync::Arc;
// use std::thread::JoinHandle;
// use std::rc::Rc;

// use shared_library::dynamic_library::DynamicLibrary;

use gtk::glib;
use gtk::prelude::*;
// use glib::clone;

// use alacritty_terminal::sync::FairMutex;
// use alacritty_terminal::event_loop::{Notifier, EventLoop, State};
// use alacritty_terminal::config::Config;
// use alacritty_terminal::tty::{self, Pty};
// use alacritty_terminal::Term;
// use alacritty_terminal::term::SizeInfo;

// Implementation of our terminal GObject
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

    // Trait shared by all GLAreas
    impl GLAreaImpl for Terminal {}
}

glib::wrapper! {
    pub struct Terminal(ObjectSubclass<imp::Terminal>)
        @extends gtk::GLArea, gtk::Widget;
}

impl Terminal {
    // Creates a new Terminal with default configurations.
    pub fn new() -> Self {
        let gl_area = glib::Object::new(&[]).expect("Failed to create Button");

        return gl_area;
    }
    // pub fn with_config() {}
    // pub fn set_font_size() {}
    // pub fn set_config(&self) {}
}