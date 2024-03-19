// Uncomment this block to pass the first stage
use std::{fs::read_to_string, io::{Read, Write}, net::{TcpListener, TcpStream}};
use std::error::Error;
use std::thread;
use std::env::args;

fn main() -> Result<(), Box<dyn Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221")?;
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let res = respond(&mut stream);
                    match res {
                        Ok(_) => {},
                        Err(err) => println!("{}", err.to_string())
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}

fn gen_200_response<T: std::fmt::Display>(data: T, len: usize) -> String {
    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", len, data)
}

fn gen_200_response_file<T: std::fmt::Display>(data: T, len: usize) -> String {
    format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", len, data)
}

fn gen_404_response() -> String {
    format!("HTTP/1.1 404 OK\r\n\r\n")
}

fn respond(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut data = [0; 2048];              
    stream.read(&mut data)?;
    let data = String::from_utf8(data.to_vec())?.split("\r\n").map(|x| x.to_string()).collect::<Vec<String>>();
    let path = data[0].split(' ').collect::<Vec<&str>>()[1];
    println!("{}", path);

    match path {
        "/" => stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?,
        "/user-agent" => {
            let user_agent = &data[2].split(": ").collect::<Vec<&str>>()[1];
            stream.write_all(gen_200_response(&user_agent, user_agent.len()).as_bytes())?
        },
        _ => {
            if path.starts_with("/echo") {
                let to_echo: String = path.split('/').collect::<Vec<&str>>()[2..].join("/");
                let data = gen_200_response(&to_echo, to_echo.len());
                stream.write_all(data.as_bytes())?;
            } else if path.starts_with("/files") {
                let dir = args().collect::<Vec<String>>()[2].to_string();
                let path = path.split('/').collect::<Vec<&str>>()[2..].join("/");
                println!("{}{}", dir, path);
                let to_send = read_to_string(format!("{}{}", dir, path)).unwrap_or("Not Found".into());
                if &to_send == &"Not Found".to_string() {
                    stream.write_all(gen_404_response().as_bytes())?
                } else {
                    stream.write_all(gen_200_response_file(&to_send, to_send.len()).as_bytes())?
                }
            } else {
                stream.write_all(b"HTTP/1.1 404 NOT_FOUND\r\n\r\n")?
            }
        }
    }
    Ok(())
}