use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use itertools::Itertools;

enum FileHandlingMode<'a> {
    Read,
    Write(&'a str),
}

fn file_handler(_path: &str, mut _stream: TcpStream, mode: FileHandlingMode) {
    let path_arr: Vec<&str> = _path.split("/").collect_vec();

    match mode {
        FileHandlingMode::Read => {
            let dir_file: Result<File, std::io::Error> = File::open(format!(
                "/tmp/data/codecrafters.io/http-server-tester/{}",
                path_arr[2]
            ));

            match dir_file {
                Ok(mut dir_file) => {
                    let mut file_content: String = String::new();
                    let bytes: usize = dir_file.read_to_string(&mut file_content).unwrap();
                    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", bytes, file_content);
                    let _ = _stream.write(response.as_bytes());
                }
                Err(_) => {
                    let _ = _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n");
                }
            }
        }
        FileHandlingMode::Write(_) => {
            let new_file: Result<File, std::io::Error> = File::create_new(path_arr[2]);

            match new_file {
                Ok(mut new_file) => {}
                Err(_) => {
                    let _ = _stream.write(b"HTTP/1.1 500 500 Internal Server Error\r\n\r\n");
                }
            }
        }
    }
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let stream_result_bytes: std::io::Bytes<TcpStream> = _stream.try_clone().unwrap().bytes();
                let stream_result_bytes_vec: Vec<Result<u8, std::io::Error>> =
                    stream_result_bytes.collect_vec();
                let mut buf: Vec<_> = Vec::new();
                for u8_result in stream_result_bytes_vec {
                    let chunk: u8 = u8_result.unwrap();
                    buf.push(chunk);
                }
                let _ = _stream.read(&mut buf);
                let request: std::borrow::Cow<str> = String::from_utf8_lossy(&buf);
                let req_lexemes: std::str::SplitWhitespace = request.split_whitespace();
                let req_lexemes_vec: Vec<&str> = req_lexemes.collect_vec();
                let _path: &str = req_lexemes_vec[1];
                let verb: &str = req_lexemes_vec[0];

                println!(
                    "req split by sig: {:?}",
                    request.split("\r\n").collect_vec()
                );

                match _path.chars().next().unwrap() {
                    '/' => {
                        let split_segs: Vec<&str> =
                            _path.split("/").filter(|seg: &&str| *seg != "").collect();
                        if split_segs.len() == 0 {
                            let _ = _stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
                        } else if split_segs.len() == 1 {
                            if split_segs[0] == "user-agent" {
                                let req_lines: Vec<&str> = request.split("\r\n").collect_vec();
                                let user_agent = req_lines[2];
                                let body = user_agent.split(": ").collect_vec()[1];
                                let content_length = body.len();
                                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", content_length, body);
                                let _ = _stream.write(response.as_bytes());
                            } else {
                                let _ = _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n");
                            }
                        } else {
                            if _path.starts_with("/files") {
                                if verb == "GET" {
                                    file_handler(_path, _stream, FileHandlingMode::Write(""));
                                } else if verb == "POST" {
                                    file_handler(_path, _stream, FileHandlingMode::Read);
                                }
                            } else {
                                let body: &str = split_segs[1];
                                let content_length = split_segs[1].len();
                                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", content_length, body);
                                let _ = _stream.write(response.as_bytes());
                            }
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
