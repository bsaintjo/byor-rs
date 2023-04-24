use std::{collections::HashMap, os::fd::AsRawFd};

use mio::{unix::SourceFd, Events, Interest, Poll, Token};
use socket2::Socket;

use crate::{startup, read_msg, send_msg};

pub fn event_loop() -> Result<(), Box<dyn std::error::Error>> {
    let mut clients: HashMap<Token, Socket> = HashMap::new();
    let server = startup::server_init()?;
    const SERVER: Token = Token(0);
    let mut index = 1;
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);
    poll.registry().register(
        &mut SourceFd(&server.as_raw_fd()),
        SERVER,
        Interest::READABLE,
    )?;
    loop {
        poll.poll(&mut events, None)?;
        for event in events.iter() {
            match event.token() {
                SERVER => {
                    log::info!("Server event");
                    let (client_socket, _) = server.accept()?;
                    poll.registry().register(
                        &mut SourceFd(&client_socket.as_raw_fd()),
                        Token(index),
                        Interest::READABLE,
                    )?;
                    clients.insert(Token(index), client_socket);
                    index = index.wrapping_add(1);
                }
                client_token if event.is_readable() => {
                    log::info!("Reading from client");
                    let source = clients.get(&client_token).unwrap();
                    let msg = read_msg(source)?;
                    poll.registry().reregister(
                        &mut SourceFd(&source.as_raw_fd()),
                        client_token,
                        Interest::WRITABLE,
                    )?;
                    println!("Message from client: {}", msg);
                }
                client_token if event.is_writable() => {
                    log::info!("Writing to client");
                    let source = clients.get(&client_token).unwrap();
                    send_msg(source, "Hello from server")?;
                    poll.registry()
                        .deregister(&mut SourceFd(&source.as_raw_fd()))?;
                }
                client_token => {
                    log::warn!("File descriptor for Token {client_token:?} is neither readable nor writable, skipping...");
                }
            }
        }
    }
    Ok(())
}
