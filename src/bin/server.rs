use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let peer = stream.peer_addr().ok();
    let mut buf = [0u8; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                // connection closed
                break;
            }
            Ok(n) => {
                // echo back
                let _ = stream.write_all(&buf[..n]);
                println!(
                    "echoed {} bytes {}",
                    n,
                    peer.map(|p| format!("to {}", p)).unwrap_or_default()
                );
            }
            Err(e) => {
                eprintln!("read error: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    // Allow overriding address via CLI args
    // Usage: cargo run --bin server -- [ADDR]
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:4000".to_string());
    let listener = TcpListener::bind(&addr)?;
    println!("server listening on {}", addr);

    // Accept connections and handle each in its own thread
    for stream in listener.incoming() {
        match stream {
            Ok(stream_obj) => {
                thread::spawn(|| handle_client(stream_obj));
            }
            Err(e) => eprintln!("accept error: {}", e),
        }
    }

    Ok(())
}


