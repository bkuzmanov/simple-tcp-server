use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 16;

fn read_message(mut stream: &TcpStream) -> Result<String, &'static str> {
    let mut buff = vec![0; MSG_SIZE];
    
    match stream.read(&mut buff) {
        Ok(_) => {
            let message = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
            let message = String::from_utf8(message).expect("Invalid UTF8");
            
            Ok(message)
        },
        Err(_) => Err("Couldn't read the stream"),
    }
}

fn handle_connection(mut stream: TcpStream) {

    let message = read_message(&stream).expect("Message couldn't not be read");

    let addr = stream.peer_addr()
                     .expect("Couldn't get peer address");
    
    println!("{}: {}", addr, message);
            
    if message == String::from("Hello There!\n") {
        stream.write(b"General Kenoby!\r\n")
              .expect("Couldn't write message to stream");
    } else {
        stream.write(b"You are not general Kenoby!\r\n")
              .expect("Couldn't write message to stream");
    }
}

fn main() {

    let listener = TcpListener::bind(LOCAL).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            },
            Err(_) => println!("Connection error"),
        }
    }
}
