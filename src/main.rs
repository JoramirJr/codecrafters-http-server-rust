use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                // let mut request = String::new();
                // let _ = _stream.read_to_string(&mut request);

                // let mut req_tokens = request.split_whitespace();
                // let _ = req_tokens.next();
                // let _path = req_tokens.next().unwrap();

                // match _path {
                //     "/" => {
                //         let _ = _stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
                //     }
                //     _ => {
                //         let _ = _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n");
                //     }
                // }
                let _ = _stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
