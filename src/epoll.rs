use epoll::Event;
use epoll::{ControlOptions::EPOLL_CTL_ADD, Events};
use std::{collections::HashMap, os::fd::AsRawFd};

use crate::{
    connection::{Connection, State},
    startup,
};

pub fn event_loop() -> Result<(), Box<dyn std::error::Error>> {
    let socket = startup::server_init()?;

    let poller = epoll::create(false)?;

    let mut connections: HashMap<u64, Connection> = Default::default();
    let server_key = socket.as_raw_fd().try_into().unwrap();
    let server_event = Event::new(Events::EPOLLIN, server_key);
    epoll::ctl(poller, EPOLL_CTL_ADD, socket.as_raw_fd(), server_event)?;

    let mut events = [Event::new(Events::empty(), 0); 4096];

    loop {
        let n = epoll::wait(poller, -1, &mut events)?;
        for &ev in events[..n].iter() {
            #[allow(unaligned_references)]
            let event_key = ev.data;

            // Accept new connection
            if event_key == server_key {
                let Ok((accepted_socket, _)) = socket.accept() else { continue; };
                let conn = Connection::new(accepted_socket);
                let conn_key = conn.fd_u64()?;
                let new_conn_event = Event::new(Events::EPOLLIN, conn_key);
                epoll::ctl(
                    poller,
                    epoll::ControlOptions::EPOLL_CTL_ADD,
                    conn.socket.as_raw_fd(),
                    new_conn_event,
                )?;
                connections.insert(conn_key, conn);
            } else {
                // Do stuff with ready connections

                let state = {
                    let connevent = connections.get_mut(&event_key).unwrap();
                    connevent.conn_io()?;
                    connevent.state
                };

                if state == State::End {
                    let (_, connevent) = connections.remove_entry(&event_key).unwrap();
                    epoll::ctl(
                        poller,
                        epoll::ControlOptions::EPOLL_CTL_DEL,
                        connevent.socket.as_raw_fd(),
                        Event::new(Events::empty(), 0),
                    )?;
                }
            }
        }
    }
}
