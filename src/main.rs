use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::current;
use rppal::gpio::Gpio;
const LED_PIN: u8 = 17;
fn handle_client(mut stream: TcpStream, led: Arc<Mutex<rppal::gpio::OutputPin>>, state: Arc<Mutex<bool>>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    loop {
        let mut buffer = String::new();
        if(reader.read_line(&mut buffer).unwrap() == 0) {
            break;
        }

        let command = buffer.trim();
        println!("Received {}", command);
        let mut pin = led.lock().unwrap();
        let mut current_state = state.lock().unwrap();

        let response = match command {
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
    let gpio = Gpio::new().expect("Failed to access GPIO");
    let led_pin = gpio.get(LED_PIN).unwrap().into_output();

    let led = Arc::new(Mutex::new(led_pin));
    let led_state = Arc::new(Mutex::new(false));

    let listener = TcpListener::bind("0.0.0.0:8000").expect("Could not bind");
    println!("Listening 0.0.0.0:8000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let led = Arc::clone(&led);
                let state = Arc::clone(&led_state);
                std::thread::spawn(move || handle_client(stream, led, state));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}