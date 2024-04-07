use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let buf_reader = BufReader::new(&mut stream);

                let _request: Vec<_> = buf_reader
                    .lines()
                    .map(|l| l.unwrap())
                    .take_while(|l| !l.is_empty())
                    .collect();

                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap()
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
