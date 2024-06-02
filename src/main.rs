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
                let mut buf = Vec::new();
                let _ = _stream.read_to_end(&mut buf);
                let request = String::from_utf8(buf);

                match request {
                    Ok(mut _request) => {
                        //exp: GET /banana HTTP/1.1\r\nHost: localhost:4221\r\n\r\n
                        let mut req_tokens = _request.split_whitespace();
                        let first_seg = req_tokens.next();
                        println!("First Segment: {:?}", first_seg);
                        let second_seg = req_tokens.next();
                        println!("2nd Segment: {:?}", second_seg);

                        // let _path = req_tokens.next();

                        // match path {
                        //     "/" => {
                        //         let _ = _stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
                        //     }
                        //     _ => {
                        //         let _ = _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n");
                        //     }
                        // }
                    }
                    Err(e) => {
                        println!("error: {}", e);
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
