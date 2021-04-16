// Special thanks to https://github.com/unrelentingtech/galacritty/ for inspiration.
use std::sync::Arc;
use std::default::Default;
use std::path::PathBuf;

use gtk::glib;
use gtk::prelude::*;

use alacritty_terminal::config::{Config, Selection, Scrolling};
use alacritty_terminal::sync::FairMutex;
use alacritty_terminal::term::{SizeInfo, Term};
use alacritty_terminal::tty;
use alacritty_terminal::event_loop::{EventLoop, Notifier};
use alacritty_terminal::event::{Event, EventListener};
pub use alacritty_terminal::config::Program;

// use glutin::event_loop::EventLoop as GlutinEventLoop;

mod config;

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
    pub fn new(program: Program, working_directory: Option<PathBuf>) -> Self {
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



            // Create a SizeInfo from gl_area for our new Term.
            let h_size = gl_area.get_size(gtk::Orientation::Horizontal);
            let v_size = gl_area.get_size(gtk::Orientation::Vertical);

            let size = SizeInfo::new(h_size as f32, v_size as f32, 3.0, 3.0, 0.0, 0.0, false);
            let config = config::get_config(program, working_directory);

            // Create a Term from our config and sizes that we created.
            let terminal = Term::new(&config, size, Notifier);
            let terminal = Arc::new(FairMutex::new(terminal));

            let pty = tty::new(&config, &size, None);

            // let event_loop = EventLoop::new(
            //     Arc::clone(&terminal),
            //     *Box::new(Listen),
            //     pty,
            //     false,
            //     false,
            // );
    
            // let myself = Self {
            //     config,
            //     size,
            // };

        });

        return gl_area;
    }
    // pub fn with_config() {}
    // pub fn set_font_size() {}
    // pub fn set_config(&self) {}
}
