// Uncomment this block to pass the first stage
use std::{io::{Read, Write}, net::TcpListener};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221")?;
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut data: &mut [u8] = &mut [0; 2048];              
                stream.read(&mut data)?;
                let data = String::from_utf8(data.to_vec())?.split("\r\n").map(|x| x.to_string()).collect::<Vec<String>>();
                let path = data[0].split(' ').collect::<Vec<&str>>()[1];
                println!("{}", path);
                if path != "/" {
                    stream.write_all(b"HTTP/1.1 404 NOT_FOUND\r\n\r\n")?;
                } else {
                    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
