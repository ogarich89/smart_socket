use std::io;
use std::str::from_utf8;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    match TcpStream::connect("127.0.0.1:3000").await {
        Ok(mut stream) => loop {
            println!("Select option: \n\r 1. Get status \n\r 2. Toggle socket \n\r 3. Exit");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            let selected = buf.trim();
            stream.write_all(selected.as_bytes()).await.unwrap();

            if selected == "3" {
                break;
            }

            let mut data = [0_u8; 32];
            match stream.read_exact(&mut data).await {
                Ok(_) => {
                    println!("{}", from_utf8(&data).unwrap().trim_matches(char::from(0)));
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Goodbye!");
}
