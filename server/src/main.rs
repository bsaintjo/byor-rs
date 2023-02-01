use std::{
    env,
    io::{self, Read, Write},
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
        do_something(&mut accepted_socket)?;
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

const K_MAX_MSG: usize = 4096;

fn one_request(accepted_socket: &mut socket2::Socket) -> Result<(), Box<dyn std::error::Error>> {
    let mut read_buffer = [0u8; 4 + K_MAX_MSG + 1];
    accepted_socket.read_exact(&mut read_buffer[..4])?;
    let len: [u8; 4] = read_buffer[..4].try_into().unwrap();
    let len: u32 = unsafe { std::mem::transmute(len) };
    if len > (K_MAX_MSG as u32) {
        return Err(format!("len {len} is longer than K_MAX_MSG: {K_MAX_MSG}").into());
    }
    let msg_span = 4..4 + (len as usize);
    accepted_socket.read_exact(&mut read_buffer[msg_span.clone()])?;
    let client_response = std::str::from_utf8(&read_buffer[msg_span])?;
    println!("Client says: {client_response}");

    const REPLY: &[u8; 5] = b"world";
    let mut write_buffer = [0u8; 4 + REPLY.len()];
    let reply_len: [u8; 4] = (REPLY.len() as u32).to_be_bytes();
    (&mut write_buffer[..4]).write_all(&reply_len)?;
    (&mut write_buffer[4..]).write_all(REPLY)?;
    accepted_socket.write_all(&write_buffer)?;

    Ok(())
}

fn receive_client_message<R>(src: &mut R) -> Result<String, Box<dyn std::error::Error>>
where
    R: Read,
{
    let mut read_buffer = [0u8; 4 + K_MAX_MSG + 1];
    src.read_exact(&mut read_buffer[..4])?;
    let len: [u8; 4] = read_buffer[..4].try_into().unwrap();
    let len: u32 = unsafe { std::mem::transmute(len) };
    if len > (K_MAX_MSG as u32) {
        return Err(format!("len {len} is longer than K_MAX_MSG: {K_MAX_MSG}").into());
    }
    let msg_span = 4..4 + (len as usize);
    src.read_exact(&mut read_buffer[msg_span.clone()])?;
    let client_response = std::str::from_utf8(&read_buffer[msg_span])?;
    Ok(client_response.to_string())
}

fn send_client_world<W>(dst: &mut W) -> Result<(), Box<dyn std::error::Error>>
where
    W: Write,
{
    const REPLY: &[u8; 5] = b"world";
    let mut write_buffer = [0u8; 4 + REPLY.len()];
    let reply_len: [u8; 4] = (REPLY.len() as u32).to_be_bytes();
    (&mut write_buffer[..4]).write_all(&reply_len)?;
    (&mut write_buffer[4..]).write_all(REPLY)?;
    dst.write_all(&write_buffer)?;
    Ok(())
}
