use stm_add;
use std::{env, fs};


fn main() {
    let config = stm_add::Config::user_input_to_config(env::args());
    let desktop_file_str = stm_add::Config::config_to_desktop_file_str(&config);
    fs::write(config.desktop_dir, desktop_file_str).expect("Failed to write your desktop file.");
}
