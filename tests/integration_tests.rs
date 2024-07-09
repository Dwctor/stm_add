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
