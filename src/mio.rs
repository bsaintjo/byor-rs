use std::os::fd::AsRawFd;

use mio::{unix::SourceFd, Interest, Poll, Token};

use crate::startup;

pub fn event_loop() -> Result<(), Box<dyn std::error::Error>> {
    let server = startup::server_init()?;
    let mut index = 0;
    let server_token = Token(index);
    index += 1;
    let poll = Poll::new()?;
    poll.registry().register(
        &mut SourceFd(&server.as_raw_fd()),
        server_token,
        Interest::READABLE,
    )?;
    Ok(())
}
