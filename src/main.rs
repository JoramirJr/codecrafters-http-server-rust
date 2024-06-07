use std::{
    borrow::Borrow,
    io::{Read, Write},
    net::TcpListener,
};

use itertools::Itertools;

struct ExtractStrAndLenReturn<'a> {
    body: &'a str,
    content_length: usize,
}

fn extract_str_and_len(route_segments: Vec<&str>) -> ExtractStrAndLenReturn {
    ExtractStrAndLenReturn {
        body: route_segments[1],
        content_length: route_segments[1].len(),
    }
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                 let stream_bytes = _stream
                     .borrow()
                     .bytes();
                //     .map(|bytes_result: Result<u8, std::io::Error>| bytes_result.unwrap())
                //     .collect_vec();
                // let request = String::from_utf8(stream_bytes).unwrap();
                // let mut req_tokens = request.split_whitespace();
                // let _ = req_tokens.next();
                // let _path = req_tokens.next().unwrap();

                // match _path.chars().next().unwrap() {
                //     '/' => {
                //         let split_segs: Vec<&str> =
                //             _path.split("/").filter(|seg| *seg != "").collect();
                //         if split_segs.len() == 1 {
                //             let _ = _stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
                //         } else {
                //             let ExtractStrAndLenReturn {
                //                 body,
                //                 content_length,
                //             } = extract_str_and_len(split_segs);
                //             //  HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 3\r\n\r\nabc
                //             let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", content_length, body);
                //             println!("Response: {:?}", response);
                //             let _ = _stream.write(response.as_bytes());
                //         }
                //     }
                //     _ => {
                //         let _ = _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n");
                //     }
                // }
                let _ = _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
