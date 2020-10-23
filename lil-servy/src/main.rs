use std::net::{TcpListener, TcpStream};
use std::io::Read;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:42451")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}