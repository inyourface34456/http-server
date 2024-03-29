// Uncomment this block to pass the first stage
use std::{fs::{read_to_string, File}, io::{Read, Write}, net::{TcpListener, TcpStream}};
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

fn gen_response(code: usize) -> String {
    format!("HTTP/1.1 {} OK\r\n\r\n", code)
}

fn respond(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    //let reader = BufReader::new(stream);
    let mut data = [0; 2048];              
    stream.read(&mut data)?;
    let data = String::from_utf8(data.to_vec())?.split("\r\n").map(|x| x.to_string()).collect::<Vec<String>>();
    let req_line = data[0].split(' ').collect::<Vec<&str>>();
    let path = req_line[1];
    let req_type = req_line[0];
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
                let path = format!("{}{}", dir, path);
//
                if req_type == "GET" {
                    if let Ok(to_send) = read_to_string(path) {
                        stream.write_all(gen_200_response_file(&to_send, to_send.len()).as_bytes())?
                    } else {
                        stream.write_all(gen_response(404).as_bytes())?
                    }
                } else if req_type == "POST" {
                    let to_write = &data[data.len()-1].trim_end_matches('\x00');
                    File::create(path)?.write_all(to_write.as_bytes())?;
                    stream.write_all(b"HTTP/1.1 201 OK\r\n\r\n")?
                }
            } else {
                stream.write_all(b"HTTP/1.1 404 NOT_FOUND\r\n\r\n")?
            }
        }
    }
    Ok(())
}