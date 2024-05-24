use core::panic;
use std::fs;
use std::path::PathBuf;

// The configuration struct contains the information needed to do the final .desktop creation.
#[derive(Default)]
pub struct Config {
    pub file_dir_str: String, // Original file of the program that will have a dotfile
    pub file_dir: PathBuf,
    pub file_name: String,
    pub desktop_folder: String, // Folder to save the .desktop file. // Consider changing to PathBuf
    pub desktop_dir: String, // Name for the .desktop file
    // Optional fields.
    pub generic_name: Option<String>,
    pub no_display: Option<String>,
    pub comment: Option<String>,
    pub icon: Option<String>, // Consider changing to PathBuf
    pub deleted: Option<bool>,
    pub only_show_in: Option<Vec<String>>,
    pub not_show_in: Option<Vec<String>>,
    pub dbus_activatable: Option<bool>,
    pub try_exec: Option<String>,
    pub working_dir_path: Option<String>, // Consider changing to PathBuf
    pub terminal: Option<bool>,
    pub actions: Option<Vec<String>>,
    pub mime_tipes: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub implements: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
    pub startup_notify: Option<bool>,
    pub startup_wm_class: Option<String>,
    pub url: Option<String>, // Move to is_url and URL
    pub prefers_non_default_gpu: Option<bool>,
    pub singlemainwindow: Option<bool>,
}

// ------------------------------------
// Public API
// ------------------------------------

impl Config {
    pub fn new() -> Self {
        Config{..Config::default()}
    }

    pub fn user_input_to_config(user_input: Vec<String>) -> Config {
        validate_user_input(&user_input);
        let file_dir_str = &user_input[1];
        let file_dir = fs::canonicalize(&file_dir_str).expect("Failed to canonicalize file path.");
        let file_dir_str: String = file_dir.to_str().unwrap().to_string();
        // Todo: make get_program_name into the private API
        let file_name = get_file_name_from_dir(&file_dir);
        let desktop_folder = "~/.local/share/applications/".to_string();
        let desktop_dir = format!("{}{}.desktop", desktop_folder, file_name); 

        Config{
            file_dir_str,
            file_dir,
            file_name,
            desktop_folder,
            desktop_dir,
            ..Config::default()
        }
    }

    // Method to create a .desktop file from a Config.
    pub fn config_to_desktop_file_str(config: &Config) -> String {
        // Format follows https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#recognized-keys
        let title = 
"[Desktop Entry]
# This file has been written by stm_add.
# stm_add creates files following the format of:
# https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#recognized-keys\n
";

        let required_values = format!(
"# -----------------------------------------------------------
# Required Values
# -----------------------------------------------------------
Type = {}
Name = {}
Exec = {}
# URL is for links.
# URL = _insert_link_here_ \n
", "Application", config.file_name , config.file_dir_str);

        let optional_values = 
"# -----------------------------------------------------------
# Optional Values
# -----------------------------------------------------------

"; // Implement optional values here

        format!("{}{}{}", title, required_values, optional_values)
    }
}

// ------------------------------------
// Private API
// ------------------------------------

fn get_file_name_from_dir(program_dir: &PathBuf) -> String {
    use std::ffi::OsStr;
    let file_name: &OsStr = match program_dir.file_name() {
        Some(x) => x,
        None => panic!("File name resolution failed.")
    };

    let file_name: &str = match file_name.to_str() {
        Some(x) => x,
        None => panic!("Failed conversion from OsStr to str")
    };

    file_name.to_string()
}

fn validate_user_input(user_input: &Vec<String>) {
    // Current user input validation follows one argument only.
    if user_input.len() > 2 {
        panic!("Wrong number of arguments.")
    }

    let program_dir = PathBuf::from(user_input[1].clone());
    if !program_dir.is_file(){
        panic!("File path invalid.")
    }
}
