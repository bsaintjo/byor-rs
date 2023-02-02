use std::{
    io::{Read, Write},
    net::SocketAddr,
};

use socket2::{Domain, Socket, Type};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let mut socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    let addr: SocketAddr = "127.0.0.1:1234".parse()?;
    socket.connect(&addr.into())?;

    let msg = b"hello";
    socket.write_all(msg.as_ref())?;

    let mut rbuf = [0u8; 64];
    match socket.read(&mut rbuf) {
        Ok(n) => log::info!("Read {n} bytes"),
        Err(e) => log::error!("Failed to read: {e}"),
    }

    println!("Server says: {}", std::str::from_utf8(&rbuf)?);

    Ok(())
}
