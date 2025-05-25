use crate::{config};

const PREFIX: &str = "[DEBUG]";

pub fn log(message: &str) {
    if config::is_debug() {
        println!("{PREFIX} {}", message);
    }
}

pub fn warn(message: &str) {
    if config::is_debug() {
        eprintln!("{PREFIX} {}", message);
    }
}