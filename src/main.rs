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
        match path.as_str() {
            "/" => stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap(),
            p if p.starts_with("/echo") => handle_echo(stream, p),
            _ => stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap(),
        }
    }
}

fn handle_echo(mut stream: TcpStream, path: &str) {
    let str = path.chars().skip(6).collect::<String>();
    let len = str.len();
    let mut response = String::from("HTTP/1.1 200 OK\r\n");
    response.push_str("Content-Type: text/plain\r\n");
    response.push_str(&format!("Content-Length: {len}\r\n\r\n{str}"));

    stream.write_all(response.as_bytes()).unwrap();
}
