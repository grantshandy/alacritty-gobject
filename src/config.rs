use alacritty_terminal::config::{Config, Program, Scrolling, Selection, Cursor};
use std::collections::HashMap;
use std::path::PathBuf;

pub fn get_config(program: Program, working_directory: Option<PathBuf>) -> Config {
    let env: HashMap<String, String> = HashMap::new();
    let selection = Selection::default();
    let shell = Some(program);
    let scrolling = Scrolling::default();
    let cursor = Cursor::default();
    let hold = false;


    Config {
        env,
        selection,
        shell,
        scrolling,
        cursor,
        working_directory,
        ui_config,
        hold,
    };
}