use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
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

fn handle_client(mut stream: TcpStream, addr: SocketAddr, mut smart_socket: SmartSocket) {
    let mut data = [0_u8; 50];
    loop {
        match stream.read(&mut data) {
            Ok(_) => {
                let result = match from_utf8(&data).unwrap().trim_matches(char::from(0)) {
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
                    _ => String::from("Unknown command!"),
                };
                data = [0_u8; 50];
                let _ = stream.write_all(result.as_bytes()).is_ok();
            }
            Err(_) => {
                println!("An error occurred, terminating connection with {}", addr);
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
                let addr = stream.peer_addr().unwrap();
                println!("New connection: {}", addr);
                thread::spawn(move || handle_client(stream, addr, smart_socket));
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}
