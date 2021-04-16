// Special thanks to https://github.com/unrelentingtech/galacritty/ for inspiration, and, admittedly, code :)
use std::sync::Arc;

use glib::clone;
use gtk::glib;
use gtk::prelude::*;

// use shared_library::dynamic_library::DynamicLibrary;

use alacritty_terminal::config::Config;
use alacritty_terminal::sync::FairMutex;
use alacritty_terminal::term::{SizeInfo, Term};
use alacritty_terminal::tty;

// Implementation of our terminal GObject
mod imp {
    // Import parent scope
    use super::*;
    // Import necessary traits for subclassing
    use gtk::subclass::prelude::*;

    // Object holding the state
    #[derive(Default)]
    pub struct Terminal {
        pub config: Config,
        pub size: SizeInfo,
    }

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
        let gl_area: Terminal = glib::Object::new(&[]).expect("Failed to create Terminal Widget");

        gl_area.connect_realize(move |gl_area| {
            gl_area.make_current();
            match gl_area.get_error() {
                Some(error) => {
                    println!("gtk::GLArea error: {}", error);
                    return;
                }
                None => {}
            }

            gl_area.set_has_depth_buffer(true);

            // epoxy::load_with(|s| {
            //     unsafe {
            //         match DynamicLibrary::open(None).unwrap().symbol(s) {
            //             Ok(v) => v,
            //             Err(_) => std::ptr::null(),
            //         }
            //     }
            // });
            // gl::load_with(epoxy::get_proc_addr);

            let config = Config::default();

            let v_size = gl_area.get_size(gtk::Orientation::Vertical);
            let h_size = gl_area.get_size(gtk::Orientation::Horizontal);

            let size = SizeInfo::new(h_size, v_size, 3.0, 3.0, 0, 0, false);

            let terminal = Term::new(&config, size);
            let terminal = Arc::new(FairMutex::new(terminal));

            let pty = tty::new(&config, &size, None);


        });

        return gl_area;
    }
    // pub fn with_config() {}
    // pub fn set_font_size() {}
    // pub fn set_config(&self) {}
}
