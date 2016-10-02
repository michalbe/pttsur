use std::net::{TcpListener, TcpStream};
use std::io::{Write};
use std::thread;

static HOST: &'static str = "127.0.0.1:8080";

fn handle_client(mut stream: TcpStream) {
    let headers = b"HTTP/1.1 200 OK\r\n\r\n";
    let content = b"Hello world";

    match stream.write(headers) {
        Ok(_) => println!("OK"),
        Err(e) => println!("Fail: {}", e),
    }

    match stream.write(content) {
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
