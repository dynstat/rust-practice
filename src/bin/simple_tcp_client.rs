use std::{
    io::{Read, Write},
    net::TcpStream,
    thread::sleep,
    time,
};

fn main() {
    // 1. create a client object using TcpStream, connect to the server address.
    // 2. send / recv message

    let mut inp_buf = [0; 1024];
    let mut s_stream = TcpStream::connect("127.0.0.1:4000").unwrap();
    loop {
        // stream object should be mut

        s_stream
            .write_all("sending from client".as_bytes())
            .unwrap();

        let rlen = s_stream.read(&mut inp_buf).unwrap();
        println!(
            "RECVD: {:?} of len {}, and its string format is : {}",
            &inp_buf[..rlen],
            rlen,
            String::from_utf8_lossy(&inp_buf[..rlen])
        );

        sleep(time::Duration::from_secs(2));
    }
}
