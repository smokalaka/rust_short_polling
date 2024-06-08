use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const POLLING_INTERVAL: u64 = 50;

pub fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(SERVER_ADDRESS)?;
    println!("Connected to server!");

    loop {
        stream.write_all(&[])?;

        let mut data = [0; 1024];
        let bytes_read = stream.read(&mut data)?;
        if bytes_read > 0 {
            println!("Received data: {}", String::from_utf8_lossy(&data[..bytes_read]));
        }

        thread::sleep(Duration::from_millis(POLLING_INTERVAL));
    }
}