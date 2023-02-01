use std::{
    env,
    io::{Read, Write},
    net::SocketAddr,
};

use socket2::{Domain, Socket, Type};

fn init_logger() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "trace");
    }
    pretty_env_logger::init_timed();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    log::info!("{socket:?}");
    socket.set_reuse_address(true)?;
    let addr: SocketAddr = "0.0.0.0:1234".parse()?;
    socket.bind(&addr.into())?;
    socket.listen(128)?;
    loop {
        log::info!("Waiting to accept new client");
        let (mut accepted_socket, _) = socket.accept()?;
        log::info!("New client!");
        one_request(&mut accepted_socket)?;
        log::info!("Completed interaction.")
    }
}

fn do_something(accepted_socket: &mut socket2::Socket) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0u8; 64];
    match accepted_socket.read(&mut buf) {
        Ok(n) => log::info!("Read {n} bytes."),
        Err(e) => log::error!("Read error occured: {e}"),
    }
    println!("Client says: {}", std::str::from_utf8(buf.as_ref())?);

    let response = b"world";
    match accepted_socket.write(response.as_ref()) {
        Ok(5) => log::info!("Write to socket successful"),
        Ok(n) => log::warn!("Write partial {n}"),
        Err(e) => log::error!("Write error occured: {e}"),
    }
    Ok(())
}

fn one_request(accepted_socket: &mut socket2::Socket) -> Result<(), Box<dyn std::error::Error>> {
    let client_response = byor::read_msg(accepted_socket)?;
    println!("Client says: {client_response}");
    byor::send_msg(accepted_socket, "world!")?;
    Ok(())
}