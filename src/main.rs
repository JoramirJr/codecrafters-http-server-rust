use std::{
    io::{Read, Write},
    net::TcpListener,
    fs::File
};

struct RetrieveFileReturn<'a> {
    content_type: &'a str,
    content_length: u32
};

fn retrieve_file(path: &str) -> RetrieveFileReturn {
    let dir_file: Result<File, std::io::Error> = File::open(path);

    match dir_file {
        Ok(dir_file) => {
            let mut file_bytes_buf: Vec<u8> = Vec::new();
            dir_file.read_to_end(&mut file_bytes_buf);
            RetrieveFileReturn {
                content_type: "application/octet-stream",
                content_length: 0
            }
        },
        Err(err) => {
            
        }
    }

}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
//
                let mut buf = [0; 1024];
                let _ = _stream.read(&mut buf);
                let request = String::from_utf8_lossy(&buf[..]);
                let mut req_tokens = request.split_whitespace();
                let _ = req_tokens.next();
                let _path = req_tokens.next().unwrap();

                match _path.chars().next().unwrap() {
                    '/' => {=
                        let split_segs: Vec<&str> =
                            _path.split("/").filter(|seg| *seg != "").collect();
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
