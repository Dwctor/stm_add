use stm_add::*;

fn main() {
    let config: Config = Config {target: "/usr/local/".to_string()};
    println!("{}", config.target);
}
