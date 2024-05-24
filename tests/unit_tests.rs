#[cfg(test)]
mod integration_tests {
    use stm_add::Config;
    use std::path::PathBuf;
    use std::io::Write;
    use std::fs;

    fn create_temp_file(file_name: &str, content: &str) -> String {
        let temp_dir = std::env::temp_dir();
        let temp_file_path = temp_dir.join(file_name);
        let mut f = fs::File::create(&temp_file_path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
        temp_file_path.to_str().unwrap().to_string()
    }

    fn remove_temp_file(file_path: &PathBuf) {
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn file_name_extracted_correctly_from_empty_file() {
        let path = create_temp_file("extract.empty.test", "");
        let user_input: Vec<String> = vec![
            "stm_add".to_string(),
            path.clone()
        ];
        let config = Config::user_input_to_config(user_input);
        assert_eq!(config.file_name, "extract.empty.test");
        remove_temp_file(&PathBuf::from(path));
    }

    #[test]
    fn creation_of_config_from_empty_file() {
        let path = create_temp_file("config.empty.test", "");
        let user_input: Vec<String> = vec![
            "stm_add".to_string(),
            path.clone()
        ];
        let config = Config::user_input_to_config(user_input);
        assert_eq!(config.file_dir_str, path);
        remove_temp_file(&PathBuf::from(path));
    }

    #[test]
    fn creation_of_desktop_file_str_from_empty_file() {
        let path = create_temp_file("desktop.empty.test", "");
        let user_input: Vec<String> = vec![
            "stm_add".to_string(),
            path.clone()
        ];
        let config = Config::user_input_to_config(user_input);
        let desktop_file_str = Config::config_to_desktop_file_str(&config);

        let mut title = desktop_file_str.lines().take(4);
        let line_1 = title.next().unwrap();
        let line_2 = title.next().unwrap();
        let line_3 = title.next().unwrap();
        let line_4 = title.next().unwrap();

        assert_eq!(line_1, "[Desktop Entry]");
        assert_eq!(line_2, "# This file has been written by stm_add.");
        assert_eq!(line_3, "# stm_add creates files following the format of:");
        assert_eq!(line_4, "# https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#recognized-keys");

        remove_temp_file(&PathBuf::from(path));
    }

    #[test]
    fn creation_of_config_from_standard_file() {
        let path = create_temp_file("config.test", "a".repeat(5000).as_str());
        let user_input: Vec<String> = vec![
            "stm_add".to_string(),
            path.clone()
        ];
        let config = Config::user_input_to_config(user_input);
        assert_eq!(config.file_dir_str, path);
        remove_temp_file(&PathBuf::from(path));
    }

    #[test]
    fn creation_of_desktop_file_str_from_standard_file() {
        let path = create_temp_file("desktop.test", "a".repeat(5000).as_str());
        let user_input: Vec<String> = vec![
            "stm_add".to_string(),
            path.clone()
        ];
        let config = Config::user_input_to_config(user_input);
        let desktop_file_str = Config::config_to_desktop_file_str(&config);

        let mut title = desktop_file_str.lines().take(4);
        let line_1 = title.next().unwrap();
        let line_2 = title.next().unwrap();
        let line_3 = title.next().unwrap();
        let line_4 = title.next().unwrap();

        assert_eq!(line_1, "[Desktop Entry]");
        assert_eq!(line_2, "# This file has been written by stm_add.");
        assert_eq!(line_3, "# stm_add creates files following the format of:");
        assert_eq!(line_4, "# https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#recognized-keys");

        remove_temp_file(&PathBuf::from(path));
    }

    #[test]
    fn creation_of_config_from_big_file() {
        let path = create_temp_file("config.big.test", "a".repeat(5000000).as_str());
        let user_input: Vec<String> = vec![
            "stm_add".to_string(),
            path.clone()
        ];
        let config = Config::user_input_to_config(user_input);
        assert_eq!(config.file_dir_str, path);
        remove_temp_file(&PathBuf::from(path));
    }

    #[test]
    fn creation_of_desktop_file_str_from_big_file() {
        let path = create_temp_file("desktop.big.test", "a".repeat(5000000).as_str());
        let user_input: Vec<String> = vec![
            "stm_add".to_string(),
            path.clone()
        ];
        let config = Config::user_input_to_config(user_input);
        let desktop_file_str = Config::config_to_desktop_file_str(&config);

        let mut title = desktop_file_str.lines().take(4);
        let line_1 = title.next().unwrap();
        let line_2 = title.next().unwrap();
        let line_3 = title.next().unwrap();
        let line_4 = title.next().unwrap();

        assert_eq!(line_1, "[Desktop Entry]");
        assert_eq!(line_2, "# This file has been written by stm_add.");
        assert_eq!(line_3, "# stm_add creates files following the format of:");
        assert_eq!(line_4, "# https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#recognized-keys");

        remove_temp_file(&PathBuf::from(path));
    }
}
#[cfg(test)]
mod unit_tests {
    // Tests validity of the desktop file created by Config
    #[cfg(test)]
    mod config_to_desktop_file {
        use stm_add::Config;

        #[test]
        fn title_is_up_to_standard(){
            let config = Config::new();
            let desktop_file = Config::config_to_desktop_file_str(&config);

            let mut title = desktop_file.lines().take(4);
            let line_1 = title.next().unwrap();
            let line_2 = title.next().unwrap();
            let line_3 = title.next().unwrap();
            let line_4 = title.next().unwrap();

            assert_eq!(line_1, "[Desktop Entry]");
            assert_eq!(line_2, "# This file has been written by stm_add.");
            assert_eq!(line_3, "# stm_add creates files following the format of:");
            assert_eq!(line_4, "# https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#recognized-keys");
        }

        // All desktop_files created by stm_add have to have this title:
        #[test]
        fn required_values_is_up_to_standard(){
            let config = Config::new();
            let desktop_file = Config::config_to_desktop_file_str(&config);

            let mut required_values = desktop_file.lines().skip(5).take(5);

            let line_1 = required_values.next().unwrap();
            let line_2 = required_values.next().unwrap();
            let line_3 = required_values.next().unwrap();
            let line_4 = required_values.next().unwrap();
            let line_5 = required_values.next().unwrap();

            // Checks if lines 1 and 3 are coments
            assert_eq!(line_1, "# -----------------------------------------------------------");
            assert_eq!(line_3, "# -----------------------------------------------------------");

            // Checks if line 2 is the title
            assert_eq!(line_2, "# Required Values");

            assert_eq!(&line_4[..7], "Type = ");
            assert_eq!(&line_5[..7], "Name = ");
        }

        // Currently the type field only supports application, so this unit test tests that
        // behavior.
        #[test]
        fn type_field_is_application(){
            let config = Config::new();
            let desktop_file = Config::config_to_desktop_file_str(&config);

            let type_field_line = desktop_file.lines().skip(8).next().unwrap();
            assert_eq!("Type = Application", type_field_line);
        }

        #[test]
        fn file_name_in_config_is_correct_in_desktop_file_str(){
            let config = Config{
                file_name: "Test_Name.exe".to_string(),
                ..Config::default()
            };

            let desktop_file = Config::config_to_desktop_file_str(&config);

            let type_field_line = desktop_file.lines().skip(9).next().unwrap();
            assert_eq!("Name = Test_Name.exe", type_field_line);
        }

        #[test]
        fn file_dir_str_in_config_becomes_exec_field(){
            let config = Config{
                file_dir_str: "./home/generic_user/programs/smt_add".to_string(),
                ..Config::default()
            };

            let desktop_file = Config::config_to_desktop_file_str(&config);

            let exec_field = desktop_file.lines().skip(10).next().unwrap();
            assert_eq!("Exec = ./home/generic_user/programs/smt_add", exec_field);
        }
    }

    // Tests validity of the Config struct sane defaults
    #[cfg(test)]
    mod default_config {
        use stm_add::Config;
        use std::path::PathBuf;

        #[test]
        fn has_required_file_dir_str_field() {
            let config = Config::default();
            assert_eq!(config.file_dir_str, "");
        }

        #[test]
        fn has_required_file_dir_field() {
            let config = Config::default();
            assert_eq!(config.file_dir, PathBuf::new());
        }

        #[test]
        fn has_required_file_name_field() {
            let config = Config::default();
            assert_eq!(config.file_name, "");
        }

        #[test]
        fn has_required_desktop_folder_field() {
            let config = Config::default();
            assert_eq!(config.desktop_folder, "");
        }

        #[test]
        fn has_required_desktop_dir_field() {
            let config = Config::default();
            assert_eq!(config.desktop_dir, "");
        }

        #[test]
        fn has_optional_generic_name_field() {
            let config = Config::default();
            assert_eq!(config.generic_name, None);
        }

        #[test]
        fn has_optional_no_display_field() {
            let config = Config::default();
            assert_eq!(config.no_display, None);
        }

        #[test]
        fn has_optional_comment_field() {
            let config = Config::default();
            assert_eq!(config.comment, None);
        }

        #[test]
        fn has_optional_icon_field() {
            let config = Config::default();
            assert_eq!(config.icon, None);
        }

        #[test]
        fn has_optional_deleted_field() {
            let config = Config::default();
            assert_eq!(config.deleted, None);
        }

        #[test]
        fn has_optional_only_show_in_field() {
            let config = Config::default();
            assert_eq!(config.only_show_in, None);
        }

        #[test]
        fn has_optional_not_show_in_field() {
            let config = Config::default();
            assert_eq!(config.not_show_in, None);
        }

        #[test]
        fn has_optional_dbus_activatable_field() {
            let config = Config::default();
            assert_eq!(config.dbus_activatable, None);
        }

        #[test]
        fn has_optional_try_exec_field() {
            let config = Config::default();
            assert_eq!(config.try_exec, None);
        }

        #[test]
        fn has_optional_working_dir_path_field() {
            let config = Config::default();
            assert_eq!(config.working_dir_path, None);
        }

        #[test]
        fn has_optional_terminal_field() {
            let config = Config::default();
            assert_eq!(config.terminal, None);
        }

        #[test]
        fn has_optional_actions_field() {
            let config = Config::default();
            assert_eq!(config.actions, None);
        }

        #[test]
        fn has_optional_mime_tipes_field() {
            let config = Config::default();
            assert_eq!(config.mime_tipes, None);
        }

        #[test]
        fn has_optional_categories_field() {
            let config = Config::default();
            assert_eq!(config.categories, None);
        }

        #[test]
        fn has_optional_implements_field() {
            let config = Config::default();
            assert_eq!(config.implements, None);
        }

        #[test]
        fn has_optional_keywords_field() {
            let config = Config::default();
            assert_eq!(config.keywords, None);
        }

        #[test]
        fn has_optional_startup_notify_field() {
            let config = Config::default();
            assert_eq!(config.startup_notify, None);
        }

        #[test]
        fn has_optional_startup_wm_class_field() {
            let config = Config::default();
            assert_eq!(config.startup_wm_class, None);
        }

        #[test]
        fn has_optional_url_field() {
            let config = Config::default();
            assert_eq!(config.url, None);
        }

        #[test]
        fn has_optional_prefers_non_default_gpu_field() {
            let config = Config::default();
            assert_eq!(config.prefers_non_default_gpu, None);
        }

        #[test]
        fn has_optional_singlemainwindow_field() {
            let config = Config::default();
            assert_eq!(config.singlemainwindow, None);
        }
    }

    #[cfg(test)]
    mod user_input_to_config {
        use stm_add::Config;
        use std::fs;
        use std::path::PathBuf;

        #[test]
        fn file_dir_str_is_the_same_as_the_second_argument_of_user_input() {
            let user_input: Vec<String> = vec![
                "stm_add".to_string(),
                "/usr/bin/bash".to_string()
            ];
            let config = Config::user_input_to_config(user_input);
            assert_eq!(config.file_dir_str, "/usr/bin/bash");
        }

        #[test]
        fn file_name_extracted_correctly_from_empty_file() {
            let user_input: Vec<String> = vec![
                "stm_add".to_string(),
                "/usr/bin/bash".to_string()
            ];
            let config = Config::user_input_to_config(user_input);
            assert_eq!(config.file_name, "bash");
        }

        #[test]
        fn file_dir_should_be_canonicalized() {
            let user_input: Vec<String> = vec![
                "stm_add".to_string(),
                "/usr/bin/bash".to_string()
            ];
            let config = Config::user_input_to_config(user_input);
            let file_dir = fs::canonicalize("/usr/bin/bash").expect("");
            assert_eq!(config.file_dir, file_dir);
        }

        #[test]
        fn desktop_dir_should_have_desktop_extension() {
            let user_input: Vec<String> = vec![
                "stm_add".to_string(),
                "/usr/bin/bash".to_string()
            ];
            let config = Config::user_input_to_config(user_input);
            assert!(config.desktop_dir.ends_with(".desktop"));
        }

        #[test]
        fn program_dir_should_be_absolute_path() {
            let user_input: Vec<String> = vec![
                "stm_add".to_string(),
                "/usr/bin/bash".to_string()
            ];
            let config = Config::user_input_to_config(user_input);
            assert!(config.file_dir.is_absolute());
        }

        #[test]
        fn program_dir_str_should_be_absolute_path() {
            let user_input: Vec<String> = vec![
                "stm_add".to_string(),
                "/usr/bin/bash".to_string()
            ];
            let config = Config::user_input_to_config(user_input);
            assert!(PathBuf::from(&config.file_dir_str).is_absolute());
        }

        #[test]
        fn desktop_dir_should_be_in_desktop_folder() {
            let user_input: Vec<String> = vec![
                "stm_add".to_string(),
                "/usr/bin/bash".to_string()
            ];
            let config = Config::user_input_to_config(user_input);
            let expected_desktop_dir = format!("{}{}.desktop", config.desktop_folder, config.file_name);
            assert_eq!(config.desktop_dir, expected_desktop_dir);
        }

        #[test]
        fn program_name_doesnt_contain_slashes() {
            let user_input: Vec<String> = vec![
                "stm_add".to_string(),
                "/usr/bin/bash".to_string()
            ];
            let config = Config::user_input_to_config(user_input);
            assert!(!config.file_name.contains("/"));
        }

        #[test]
        fn desktop_folder_is_one_of_the_allowed_desktop_folders() {
            let user_input: Vec<String> = vec![
                "stm_add".to_string(),
                "/usr/bin/bash".to_string()
            ];
            let config = Config::user_input_to_config(user_input);
            let allowed_folders = vec![
                "~/.local/share/applications/",
                "/usr/share/applications/",
                "/usr/local/share/applications/"
            ];
            assert!(allowed_folders.contains(&config.desktop_folder.as_str()));
        }

        #[test]
        fn desktop_dir_is_in_desktop_folder() {
            let user_input: Vec<String> = vec![
                "stm_add".to_string(),
                "/usr/bin/bash".to_string()
            ];
            let config = Config::user_input_to_config(user_input);
            assert!(config.desktop_dir.starts_with(&config.desktop_folder));
        }

        #[test]
        #[should_panic(expected = "Wrong number of arguments.")]
        fn validation_doesnt_accept_more_than_two_arguments() {
            let user_input: Vec<String> = vec![
                "stm_add".to_string(),
                "foo".to_string(),
                "bar".to_string(),
            ];
            Config::user_input_to_config(user_input);
        }

        #[test]
        #[should_panic(expected = "File path invalid.")]
        fn validation_expects_second_argument_to_be_a_file() {
            let user_input: Vec<String> = vec![
                "stm_add".to_string(),
                "/invalid/path/to/program".to_string()
            ];
            Config::user_input_to_config(user_input);
        }
    }
}

// Kept here for future reference
/*mod end_to_end_tests {
    use ;
    fn end_to_end_fixture () {
        compile_program();
    }

    compile_program(){
        cargo build
        bash.mv(build_file -> dir/test)
    }

    fn test(){

    }
}*/
