use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

fn main() {
    // A simple TCP server that just accepts one client, for simplicity.
    // 1. Listener
    // 2. create and bind address
    // 3. accept (blocking api) new client connections
    // 4. send data

    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();

    println!("Server is running on 127.0.0.1:4000");

    let (mut c_stream, c_addr) = listener.accept().unwrap();
    println!("-------- New client connected: {:?} --------", c_addr);

    loop {
        // let mut inp_buf: Vec<u8> = vec![0; 1024];
        let mut inp_buf: [u8; 1024] = [0; 1024];
        let len_inp_buf = c_stream.read(&mut inp_buf).unwrap_or(0);
        println!(
            "{:?} of size {} recvd.. \nIt's string format: {}",
            &inp_buf[..len_inp_buf],
            len_inp_buf,
            String::from_utf8_lossy(&inp_buf)
        );
        let out_buf_str = "returning from server !!!!! ";
        c_stream.write_all(out_buf_str.as_bytes()).unwrap();
    }

    // Ok(())
}
