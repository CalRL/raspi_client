use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, MutexGuard};
use rppal::gpio::{Gpio, OutputPin};
const LED_PIN: u8 = 17;
fn handle_client(mut stream: TcpStream, led: Arc<Mutex<OutputPin>>, state: Arc<Mutex<bool>>) {
    let mut reader: BufReader<TcpStream> = BufReader::new(stream.try_clone().unwrap());
    loop {
        let mut buffer: String = String::new();
        if reader.read_line(&mut buffer).unwrap() == 0 {
            break;
        }

        let command: &str= buffer.trim();
        println!("Received {}", command);
        let mut pin: MutexGuard<OutputPin> = led.lock().unwrap();
        let mut current_state: MutexGuard<bool> = state.lock().unwrap();

        let response: String = match command {
            "toggle" => {
                *current_state = !*current_state;
                pin.write(if *current_state {
                   rppal::gpio::Level::High
                } else {
                    rppal::gpio::Level::Low
                });
                format!("LED is {}\n", if *current_state { "on" } else { "off" })
            }
            "on" => {
                pin.set_high();
                *current_state = true;
                "LED turned on\n".to_string()
            }
            "off" => {
                pin.set_low();
                *current_state = false;
                "Led turned off\n".to_string()
            }
            _ => "Invalid Command\n".to_string(),
        };

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to send response: {}", e);
            break;
        }
    }
}

fn main() {
    let gpio: Gpio = Gpio::new().expect("Failed to access GPIO");
    let led_pin: OutputPin = gpio.get(LED_PIN).unwrap().into_output();

    let led: Arc<Mutex<OutputPin>> = Arc::new(Mutex::new(led_pin));
    let led_state: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    let listener: TcpListener = TcpListener::bind("0.0.0.0:8000").expect("Could not bind");
    println!("Listening 0.0.0.0:8000");
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                if let Ok(peer_addr) = stream.peer_addr() {
                    println!("New connection from {}", peer_addr);
                } else {
                    println!("New connection (unknown address)");
                }

                let led: Arc<Mutex<OutputPin>> = Arc::clone(&led);
                let state : Arc<Mutex<bool>>= Arc::clone(&led_state);
                std::thread::spawn(move || handle_client(stream, led, state));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}