use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                tokio::spawn(async move { handle_connection(stream) });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    let buf_reader = BufReader::new(&mut stream);

    let request: Vec<_> = buf_reader
        .lines()
        .map(|l| l.expect("request should never be empty"))
        .take_while(|l| !l.is_empty())
        .collect();

    if let Some(path) = request
        .first()
        .map(|l| l.split_whitespace().skip(1).take(1).collect::<String>())
    {
        if path == "/" {
            stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;
        }
        let parts = path.split('/').collect::<Vec<_>>();
        match parts[1] {
            "echo" => handle_echo(stream, &parts[2..])?,
            "user-agent" => handle_user_agent(stream, &request)?,
            "files" => handle_file_read(stream, parts[2])?,
            _ => stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n")?,
        }
    }

    Ok(())
}

fn handle_echo(mut stream: TcpStream, path: &[&str]) -> anyhow::Result<()> {
    let str = path.join("/");
    let len = str.len();
    let mut response = String::from("HTTP/1.1 200 OK\r\n");
    response.push_str("Content-Type: text/plain\r\n");
    response.push_str(&format!("Content-Length: {len}\r\n\r\n{str}"));

    stream.write_all(response.as_bytes())?;

    Ok(())
}

fn handle_user_agent(mut stream: TcpStream, req: &[String]) -> anyhow::Result<()> {
    if let Some((_, user_agent)) = req
        .iter()
        .find(|l| l.starts_with("User-Agent"))
        .and_then(|l| l.split_once(' '))
    {
        let len = user_agent.len();
        let mut response = String::from("HTTP/1.1 200 OK\r\n");
        response.push_str("Content-Type: text/plain\r\n");
        response.push_str(&format!("Content-Length: {len}\r\n\r\n{user_agent}"));

        stream.write_all(response.as_bytes())?;
    }

    Ok(())
}

fn handle_file_read(mut stream: TcpStream, path: &str) -> anyhow::Result<()> {
    if let Some(dir) = std::env::args().last() {
        let dir_path = PathBuf::from(dir);
        let file_path = dir_path.join("files").join(path);
        println!("{file_path:?}");
        if dir_path.exists() && dir_path.is_dir() {
            let file_path = dir_path.join(path);
            match File::open(file_path) {
                Ok(mut file) => {
                    let len = file
                        .metadata()
                        .expect("file metadata was not acessible")
                        .len();
                    let mut response = String::from("HTTP/1.1 200 OK\r\n");
                    response.push_str("Content-Type: application/octet-stream\r\n");
                    response.push_str(&format!("Content-Length: {}\r\n\r\n", len));
                    stream.write_all(response.as_bytes())?;
                    std::io::copy(&mut file, &mut stream)?;
                }
                Err(_) => {
                    stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
                }
            }
        } else {
            stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
        }
    }

    Ok(())
}
