use std::str::from_utf8;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

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

async fn handle_client(mut stream: TcpStream, smart_socket: Arc<Mutex<SmartSocket>>) {
    let mut data = [0_u8; 1];
    loop {
        match stream.read_exact(&mut data).await {
            Ok(_) => {
                let result = match from_utf8(&data).unwrap() {
                    "1" => smart_socket.lock().await.get_status(),
                    "2" => {
                        smart_socket.lock().await.toggle();
                        let status = if smart_socket.lock().await.is_enabled {
                            "enabled"
                        } else {
                            "disabled"
                        };
                        format!("Smart socket is {}", status)
                    }
                    "3" => {
                        println!("{} is disconnected", stream.peer_addr().unwrap());
                        stream.shutdown().await.unwrap();
                        break;
                    }
                    _ => String::from("Unknown command!"),
                };
                let mut buf = result.into_bytes();
                buf.resize(32, 0);
                stream.write_all(buf.as_slice()).await.unwrap()
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown().await.unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let smart_socket = Arc::new(Mutex::new(SmartSocket {
        is_enabled: false,
        voltage: 0,
    }));
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Can't bind tcp listener");
    println!("Server listening on port 3000");

    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection: {}", addr);

        let smart_socket = smart_socket.clone();
        tokio::spawn(async move { handle_client(stream, smart_socket).await });
    }
}
