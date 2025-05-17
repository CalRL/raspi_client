use crate::file;

const PREFIX: &str = "[DEBUG]";

pub fn is_debug() -> bool {
    let string = file::read_env_value("DEBUG", ".env").unwrap().to_lowercase();
    if string == "true" {
        true
    } else {
        false
    }
}

pub fn log(message: &str) {
    if is_debug() {
        println!("{PREFIX} {}", message);
    }
}

pub fn warn(message: &str) {
    if is_debug() {
        eprintln!("{PREFIX} {}", message);
    }
}