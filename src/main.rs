// Uncomment this block to pass the first stage
use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut data: &mut [u8] = &mut [0];
                stream.read(&mut data).expect("could not read incomming data");
                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").expect("could not send data");
                println!("{:?}", data);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
