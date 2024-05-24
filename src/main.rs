use stm_add;
use std::{env, fs};

fn main() {
    let user_input: Vec<String> = env::args().collect();
    let config = stm_add::Config::user_input_to_config(user_input);
    let desktop_file_str = stm_add::Config::config_to_desktop_file_str(&config);
    fs::write(config.desktop_dir, desktop_file_str).expect("Failed to write your desktop file.");
}
