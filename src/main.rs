use std::{io::{BufWriter, Write}, net::TcpListener};

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                let buf = BufWriter::new(response);
                println!(buf);
                // _stream.write(buf);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
