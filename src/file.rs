use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_ip() -> String {
    let value = read_env_value("HOST", ".env").unwrap_or_else(|| "192.168.0.101:8000".to_string());
    value
}

pub fn read_env_value(key: &str, path: &str) -> Option<String> {
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        // Skip comments and blank lines
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key {
                return Some(v.trim().to_string());
            }
        }
    }

    None
}