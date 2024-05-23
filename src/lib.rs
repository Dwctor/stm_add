use core::panic;
use std::{fs, env};
use std::path::PathBuf;

// The configuration struct contains the information needed to do the final .desktop creation.
pub struct Config {
    pub program_dir_str: String, // Original directory of the program that will have a dotfile
    pub program_dir: PathBuf,
    pub program_name: String,
    pub desktop_directory: String, // Where to save the .desktop file
}

// Public API

// Gathers user input and creates config file
// To test: 
    // Make sure that program_dir_str has the same dir as program_dir
    // Make sure that both program_dir_sr and program_dir are not relative directories
    // Make sure that, at this point, program_dir exists
// Todo: impl Config { this section.
pub fn user_input_to_config(program_dir_str: String) -> Config {
    let program_dir = fs::canonicalize(&program_dir_str).expect("Failed to canonicalize file path.");
    let program_dir_str: String = program_dir.to_str().unwrap().to_string();
    // Todo: make get_program_name into the private API
    let program_name = program_dir.file_name().unwrap().to_str().unwrap().to_string();
    let desktop_directory = "~/.local/share/applications/".to_string();


    Config{
        program_dir_str,
        program_dir,
        program_name,
        desktop_directory,
    }
}

pub fn validade_user_input() {
    // If there is a way in constant time to get the size of env::args(), please substitute it here
    // - Kael
    // Current user input validation follows one argument only.
    if env::args().count() > 2 {
        panic!("Wrong number of arguments.")
    }

    let program_dir = PathBuf::from(env::args().nth(1).unwrap());
    if !program_dir.is_file(){
        panic!("File path invalid.")
    }
}

// To add: create a panic method called usage.

// Method to create a .desktop file from a Config.
pub fn config_to_desktop_file_str(config: &Config) -> String {
    // Format follows https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#recognized-keys
    format!("
    [Desktop Entry]
    Type = {}
    Name = {}
    Exec = {}

    ", "Application", config.program_name , config.program_dir_str)
}
