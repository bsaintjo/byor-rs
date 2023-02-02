use std::net::SocketAddr;

use socket2::{Socket, Domain, Type};


pub fn query<B: AsRef<[u8]>>(socket: &mut Socket, text: B) -> Result<(), Box<dyn std::error::Error>> {
    byor::send_msg(socket, text)?;
    let server_response = byor::read_msg(socket)?;
    println!("Server response: {server_response}");
    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    let addr: SocketAddr = "127.0.0.1:1234".parse()?;
    socket.connect(&addr.into())?;

    query(&mut socket, "world!")?;
    query(&mut socket, "another")?;
    query(&mut socket, "message")?;
    Ok(())
}