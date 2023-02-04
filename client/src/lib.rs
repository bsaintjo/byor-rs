use std::{net::SocketAddr};

use socket2::{Socket, Domain, Type};


pub fn query<B: AsRef<[u8]>>(socket: &Socket, text: B) -> Result<(), Box<dyn std::error::Error>> {
    byor::send_msg(socket, text)?;
    let server_response = byor::read_msg(socket)?;
    println!("Server response: {server_response}");
    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let socket = Socket::new_raw(Domain::IPV4, Type::STREAM, None)?;
    let addr: SocketAddr = "127.0.0.1:1234".parse()?;
    socket.connect(&addr.into())?;
    log::info!("Connected socket {socket:?}");

    query(&socket, "hello!")?;
    query(&socket, "HeLlO")?;
    query(&socket, "olleh")?;
    Ok(())
}