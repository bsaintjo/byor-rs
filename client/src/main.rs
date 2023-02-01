use std::{
    io::{Read, Write},
    net::SocketAddr,
};

use byor::K_MAX_MSG;
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

fn query(socket: &mut Socket, text: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let len = text.len();
    if len > K_MAX_MSG {
        return Err("Failed to ".into());
    }

    let len: [u8; 4] = (len as u32).to_be_bytes();
    let mut write_buffer = [0u8; 4 + K_MAX_MSG];
    (&mut write_buffer[..4]).write_all(&len)?;
    (&mut write_buffer[4..]).write_all(text)?;

    let server_response = byor::read_msg(socket)?;
    println!("Server response: {server_response}");

    todo!()
}
