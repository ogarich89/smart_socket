use iced::widget::{Button, Column, Text};
use iced::{button, Alignment, Element, Sandbox, Settings};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

pub fn main() -> iced::Result {
    SmartSocket::run(Settings::default())
}

struct SmartSocket {
    status: String,
    button: button::State,
    stream: TcpStream,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Toggle,
}

fn send_command(command: i32, mut stream: &TcpStream) -> String {
    stream.write_all(command.to_string().as_bytes()).unwrap();
    let mut data = [0_u8; 32];
    match stream.read_exact(&mut data) {
        Ok(_) => from_utf8(&data)
            .unwrap()
            .trim_matches(char::from(0))
            .to_string(),
        Err(error) => {
            panic!("Failed to receive data: {}", error);
        }
    }
}

impl Sandbox for SmartSocket {
    type Message = Message;

    fn new() -> Self {
        let stream = TcpStream::connect("localhost:3000").unwrap();
        Self {
            status: send_command(1, &stream),
            button: Default::default(),
            stream,
        }
    }

    fn title(&self) -> String {
        String::from("SmartSocket - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Toggle => {
                let response = send_command(2, &self.stream);
                println!("{}", response);
                self.status = send_command(1, &self.stream);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let button = Button::new(&mut self.button, Text::new("Toggle")).on_press(Message::Toggle);
        let text = Text::new(&self.status).size(50);

        Column::new()
            .push(text)
            .push(button)
            .padding(20)
            .align_items(Alignment::Center)
            .into()
    }
}
