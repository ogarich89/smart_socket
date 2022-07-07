use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

#[derive(Clone, Copy)]
struct SmartSocket {
    pub is_enabled: bool,
    voltage: i32,
}

impl SmartSocket {
    pub fn toggle(&mut self) {
        self.is_enabled = !self.is_enabled;
        self.voltage = if self.is_enabled { 100 } else { 0 }
    }

    pub fn get_status(&self) -> String {
        let status = if self.is_enabled {
            "enabled"
        } else {
            "disabled"
        };
        format!("Status: {}, voltage: {}", status, self.voltage)
    }
}

fn handle_client(mut stream: TcpStream, mut smart_socket: SmartSocket) {
    let mut data = [0_u8; 1];
    loop {
        match stream.read_exact(&mut data) {
            Ok(_) => {
                let result = match from_utf8(&data).unwrap() {
                    "1" => smart_socket.get_status(),
                    "2" => {
                        smart_socket.toggle();
                        let status = if smart_socket.is_enabled {
                            "enabled"
                        } else {
                            "disabled"
                        };
                        format!("Smart socket is {}", status)
                    }
                    "3" => {
                        println!("{} is disconnected", stream.peer_addr().unwrap());
                        stream.shutdown(Shutdown::Both).unwrap();
                        break;
                    }
                    _ => String::from("Unknown command!"),
                };
                let mut buf = result.into_bytes();
                buf.resize(32, 0);
                stream.write_all(buf.as_slice()).unwrap()
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }
}

fn main() {
    let smart_socket = SmartSocket {
        is_enabled: false,
        voltage: 0,
    };
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Server listening on port 3000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream, smart_socket));
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}
