use std::{
    io::{Read, Write},
    net::TcpListener,
};

use itertools::Itertools;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");

                let mut buf = [0; 1024];
                let _ = _stream.read(&mut buf);
                let request = String::from_utf8_lossy(&buf[..]);
                let mut req_tokens = request.split_whitespace();
                let _ = req_tokens.next();
                let _path = req_tokens.next().unwrap();

                match _path.chars().next().unwrap() {
                    '/' => {
                        let split_segs: Vec<&str> =
                            _path.split("/").filter(|seg| *seg != "").collect();
                        if split_segs.len() == 0 {
                            let _ = _stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
                        } else if split_segs.len() == 1 {
                            if split_segs[0] == "user-agent" {
                                let req_lines: Vec<&str> = request.split("\r\n").collect_vec();
                                let body = req_lines[2];
                                let user_agent = req_lines[2];
                                let content_length = user_agent.split(": ").collect_vec()[1];
                                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", content_length, body);
                                let _ = _stream.write(response.as_bytes());
                            } else {
                                let _ = _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n");
                            }
                        } else {
                            let body = split_segs[1];
                            let content_length = split_segs[1].len();
                            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", content_length, body);
                            let _ = _stream.write(response.as_bytes());
                        }
                    }
                    _ => {
                        let _ = _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n");
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
