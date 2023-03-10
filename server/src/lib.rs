use std::net::SocketAddr;

use socket2::{Socket, Domain, Type};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    log::info!("{socket:?}");
    socket.set_reuse_address(true)?;
    let addr: SocketAddr = "0.0.0.0:1234".parse()?;
    socket.bind(&addr.into())?;
    socket.listen(128)?;

    loop {
        log::info!("Waiting to accept new client");
        let (accepted_socket, _) = socket.accept()?;
        log::info!("New client!");
        loop {
            log::info!("Processing request");
            if one_request(&accepted_socket).is_err() {
                break
            }
        }
        log::info!("Completed interaction.");
    }
}

fn one_request(accepted_socket: &socket2::Socket) -> Result<(), Box<dyn std::error::Error>> {
    let client_response = byor::read_msg(accepted_socket)?;
    println!("Client says: {client_response}");
    byor::send_msg(accepted_socket, "world!")?;
    Ok(())
}