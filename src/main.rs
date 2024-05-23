use stm_add;
use std::{env::args, fs};


fn main() {
    stm_add::validade_user_input();

    // unwrap is allowed here because the above function guarantees the .nth(1) to exist.
    let program_dir_str = args().nth(1).unwrap();
    let config = stm_add::user_input_to_config(program_dir_str);
    let desktop_file_str = stm_add::config_to_desktop_file_str(&config);
    fs::write(config.desktop_directory, desktop_file_str).expect("Failed to write your desktop file.");
}

