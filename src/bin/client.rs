use std::env;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Allow overriding address and message via CLI args
    // Usage: cargo run --bin client -- [ADDR] [MESSAGE]
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:4000".to_string());
    let message = env::args().nth(2).unwrap_or_else(|| "hello from client".to_string());

    println!("connecting to {}...", addr);
    let mut stream = TcpStream::connect(&addr)?;

    // Fail fast on timeout configuration issues
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;

    // Send full message
    stream.write_all(message.as_bytes())?;
    println!("sent: {:?}", message);

    // Gracefully close the write half so the server can finish and we can read EOF
    stream.shutdown(Shutdown::Write)?;

    // Read response until EOF
    let mut buf = [0u8; 1024];
    let mut total = Vec::new();
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break, // EOF
            Ok(n) => total.extend_from_slice(&buf[..n]),
            Err(e) => return Err(e),
        }
    }
    println!("recv: {:?}", String::from_utf8_lossy(&total));

    Ok(())
}