#[cfg(test)]
mod unit_tests {
    use stm_add::*;

    #[test]
    fn trivially_true_test() {
        // A test template to be deleted eventually
        let config: Config = Config {target: "/usr/share/".to_string()};
         
        assert_eq!(config.target, "/usr/share/a");
    }
}
