use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request: Vec<_> = buf_reader
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    if let Some(path) = request
        .first()
        .map(|l| l.split_whitespace().skip(1).take(1).collect::<String>())
    {
        if path == "/" {
            stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap()
        } else {
            stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap()
        }
    }
}
