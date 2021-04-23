// Special thanks to https://github.com/unrelentingtech/galacritty/ for inspiration.
use std::default::Default;
use std::sync::Arc;

use gtk::glib;
use gtk::prelude::*;

use alacritty_terminal::Term;
use alacritty_terminal::sync::FairMutex;
use alacritty_terminal::term::SizeInfo;

mod glium_widgets;

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

        gl_loader::init_gl();

        gl_area.connect_realize(move |gl_area| {
            gl_area.make_current();

            let width = gl_area.width();
            let height = gl_area.height();

            let size_info = SizeInfo::new(width as f32, height as f32, 3.0, 3.0, 0, 0, false);
            let config = Config::default();

            let terminal = Term::new(&config, size_info, );
            let terminal = Arc::new(FairMutex::new(terminal));

        });

        return gl_area;
    }
}
