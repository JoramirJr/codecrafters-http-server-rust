use std::{
    io::{Read, Write},
    net::TcpListener,
};

// fn extract_str_and_len(){

// }

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut buffer = [0; 10];
                let _ = _stream.read(&mut buffer);
                let request = String::from_utf8(buffer.to_vec()).unwrap();
                let mut req_tokens = request.split_whitespace();
                let _ = req_tokens.next();
                let _path = req_tokens.next().unwrap();

                match _path.chars().next().unwrap() {
                    '/' => {
                        let split_segs: Vec<&str> = _path.split("/").collect();
                        println!("Path: {:?}", _path);
                        //
                        let _ = _stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
                    }
                    _ => {
                        let _ = _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n");
                    }
                }
                let _ = _stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
