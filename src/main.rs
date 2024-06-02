use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut buf = Vec::new();
                _stream.read_to_end(&mut buf);
                    println!("incoming structure/HTTP: {:?}", String::from_utf8(buf));
                // let _ = _stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
