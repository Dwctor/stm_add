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

    // A test template to be deleted eventually
    #[test]
    fn trivially_true_test() {
        let var = "/usr/share/";
         
        assert_eq!(var, "/usr/share/");
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
