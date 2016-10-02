use std::net::{TcpListener, TcpStream};
use std::io::{Write};
use std::thread;

static HOST: &'static str = "127.0.0.1:8080";

fn handle_client(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\n\r\nHello world";
    match stream.write(response) {
        Ok(_) => println!("OK"),
        Err(e) => println!("Fail: {}", e),
    }
}

fn main() {
    let listener = TcpListener::bind(HOST).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => println!("Unable to read stream: {}", e),
        }
    }
    drop(listener);
}
