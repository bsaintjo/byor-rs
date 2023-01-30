use std::{
    io::{Read, Write},
    net::SocketAddr,
};

use socket2::{Domain, Socket, Type};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.set_reuse_address(true)?;
    let addr: SocketAddr = "0.0.0.0:1234".parse()?;
    socket.bind(&addr.into())?;
    socket.listen(128)?;
    loop {
        let Ok(mut sock) = socket.accept() else { continue };
        do_something(&mut sock)?;
    }
}

fn do_something(
    sock: &mut (socket2::Socket, socket2::SockAddr),
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0u8; 64];
    match sock.0.read(&mut buf) {
        Ok(64) => (),
        Ok(n) => eprintln!("Read partial {n}"),
        Err(e) => eprintln!("Read error occured: {e}"),
    }
    println!("Client says: {}", std::str::from_utf8(buf.as_ref())?);

    let response = b"world";
    match sock.0.write(response.as_ref()) {
        Ok(5) => (),
        Ok(n) => eprintln!("Write partial {n}"),
        Err(e) => eprintln!("Write error occured: {e}"),
    }
    Ok(())
}
