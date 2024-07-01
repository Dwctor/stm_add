#[cfg(test)]
mod e2e_tests {
    use std::fs;
    use assert_cmd::Command;
    use tempfile::tempdir;

    #[test]
    fn should_not_run_successfully_when_missing_argument() {
        Command::cargo_bin("stm_add")
            .unwrap()
            .assert()
            .failure();
    }

    #[test]
    fn should_not_run_successfully_with_wrong_argument() {
        Command::cargo_bin("stm_add")
            .unwrap()
            .arg("!")
            .assert()
            .failure();
    }

    #[test]
    fn should_not_run_successfully_with_too_many_arguments() {
        Command::cargo_bin("stm_add")
            .unwrap()
            .arg("a")
            .arg("a")
            .arg("a")
            .assert()
            .failure();
    }

    #[test]
    fn should_run_successfully_with_correct_argument() {
        let tmp_home_dir = tempdir().expect("Failed to create temporary directory.");
        let applications_dir = tmp_home_dir.path().join(".local/share/applications");
        fs::create_dir_all(&applications_dir).expect("Failed to create applications directory.");

        // set the home dir as tmp_home_dir
        std::env::set_var("HOME", tmp_home_dir.path());

        Command::cargo_bin("stm_add")
            .unwrap()
            .arg("/usr/bin/bash")
            .assert()
            .success();
    }

    #[test]
    fn should_create_file_when_run_successfully() {
        let tmp_home_dir = tempdir().expect("Failed to create temporary directory.");
        let applications_dir = tmp_home_dir.path().join(".local/share/applications");
        fs::create_dir_all(&applications_dir).expect("Failed to create applications directory.");

        // sleep for 5ms just to make sure that the tmp_home_dir gets created
        // without the line below this test becomes flaky
        std::thread::sleep(std::time::Duration::from_millis(5));

        // set the home dir as tmp_home_dir
        std::env::set_var("HOME", tmp_home_dir.path());

        let output_file = applications_dir.join("bash.desktop");

        Command::cargo_bin("stm_add")
            .unwrap()
            .arg("/usr/bin/bash")
            .assert()
            .success();

        assert!(output_file.exists());
    }
}
