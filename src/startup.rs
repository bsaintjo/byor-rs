use std::{error::Error, net::SocketAddr};

use socket2::{Domain, Type};

pub(crate) fn server_init() -> Result<socket2::Socket, Box<dyn Error>> {
    let socket = socket2::Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.set_reuse_address(true)?;
    socket.set_nonblocking(true)?;
    let address: SocketAddr = "0.0.0.0:1234".parse()?;
    socket.bind(&address.into())?;
    Ok(socket)
}
