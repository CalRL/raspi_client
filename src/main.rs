mod file;
mod debug;
mod config;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use rppal::gpio::{Gpio, Level};

const LED_PIN: u8 = 17;

fn main() {
    config::init();

    println!("Debug Enabled: {}", config::is_debug().to_string());
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let server_addr: String = file::get_ip();

    ctrlc::set_handler(move || {
        println!("Ctrl+C pressed. Exiting...");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler");

    let gpio = Gpio::new().expect("Failed to access GPIO");
    let mut led = gpio.get(LED_PIN).unwrap().into_output();
    let mut state = false;
    println!("Starting at: {}", &server_addr);
    while running.load(Ordering::SeqCst) {
        match TcpStream::connect(&server_addr) {
            Ok(mut stream) => {
                println!("Connected to middleman at {}", &server_addr);
                stream.set_nodelay(true).unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(1)))
                    .expect("Failed to set read timeout");

                let mut buffer = [0u8; 128];

                while running.load(Ordering::SeqCst) {
                    match stream.read(&mut buffer) {
                        Ok(0) => {
                            println!("Server closed the connection.");
                            break;
                        }
                        Ok(n) => {
                            let raw = &buffer[..n];
                            let text = match std::str::from_utf8(raw) {
                                Ok(s) => s.trim(),
                                Err(_) => {
                                    let _ = stream.write_all(b"Invalid UTF-8\n");
                                    continue;
                                }
                            };

                            debug::log(&format!("Received raw: {}", text));

                            let json = match json::parse(text) {
                                Ok(j) => j,
                                Err(e) => {
                                    eprintln!("Invalid JSON: {}", e);
                                    let _ = stream.write_all(b"Invalid JSON\n");
                                    continue;
                                }
                            };

                            let command_str = match json["content"].as_str() {
                                Some(cmd) => cmd,
                                None => {
                                    eprintln!("Missing or invalid 'command' key.");
                                    let _ = stream.write_all(b"Missing command\n");
                                    continue;
                                }
                            };
                            let msg = format!("Parsed command: {}", command_str);
                            debug::log(&*msg);

                            let response = match command_str {
                                "toggle" => {
                                    state = !state;
                                    led.write(if state { Level::High } else { Level::Low });
                                    format!("Toggled. Now {}\n", if state { "ON" } else { "OFF" })
                                }
                                "on" => {
                                    led.set_high();
                                    state = true;
                                    "Set to ON\n".to_string()
                                }
                                "off" => {
                                    led.set_low();
                                    state = false;
                                    "Set to OFF\n".to_string()
                                }
                                _ => "Unknown command\n".to_string(),
                            };

                            debug::log(&response);

                            let source = json["source"].as_str().unwrap_or("unknown");
                            let destination = json["destination"].as_str().unwrap_or("unknown");

                            let response_json = json::object! {
                                source: destination,
                                destination: source,
                                content: response,
                                "type": "RASPI"
                            };
                            let response = format!("{}\n", response_json.dump());

                            debug::log(&response);

                            if let Err(e) = stream.write_all(response.as_bytes()) {
                                eprintln!("Write error: {}", e);
                                break;
                            }
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock
                            || e.kind() == std::io::ErrorKind::TimedOut => {
                            continue;
                        }
                        Err(e) => {
                            eprintln!("Read error: {}", e);
                            break;
                        }
                    }
                }

                println!("Disconnected. Retrying in 3s...");
                std::thread::sleep(Duration::from_secs(3));
            }
            Err(e) => {
                eprintln!("Connection failed: {}. Retrying in 3s...", e);
                std::thread::sleep(Duration::from_secs(3));
            }
        }
    }

    println!("Cleaning up GPIO...");
    led.set_low();
    println!("Shutdown complete.");
}