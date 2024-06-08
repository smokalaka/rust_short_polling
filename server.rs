use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const POLLING_INTERVAL: u64 = 50;

pub fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(SERVER_ADDRESS);
    println!("Server listening on {}", SERVER_ADDRESS);

    loop {
        let (mut stream, _) = listener.accept()?;
        println!("New client connected!");

        let mut data = String::new();

        loop {
            thread::sleep(Duration::from_millis(100));
            data = "New data available!".to_string();

            stream.write_all(data.as_bytes())?;

            let mut response = [0; 1024];
            let bytes_read = stream.read(&mut response)?;

            if bytes_read > 0 {
                println!("Client resonse: {}", String::from_utf8_lossy(&response[..bytes_read]));
            }

            thread::sleep(Duration::from_millis(POLLING_INTERVAL));
            break;
        }
        stream.flush()?;
    }
}