use std::collections::HashMap;

use polling::{Event, Poller};

use crate::{connection::Connection, startup};

fn event_loop() -> Result<(), Box<dyn std::error::Error>> {
    let socket = startup::server_init()?;
    let mut event_key = 0;
    let server_key = event_key;
    event_key += 1;

    let mut connections: HashMap<usize, Box<Connection>> = Default::default();
    let poller = Poller::new()?;
    poller.add(&socket, Event::all(server_key))?;
    let mut events: Vec<Event> = Vec::new();
    loop {
        events.clear();
        poller.wait(&mut events, None)?;
        for event in &events {
            if event.key == server_key {
                log::info!("Server event");
                poller.modify(&socket, Event::all(server_key))?;

                let (accepted_socket, _) = socket.accept()?;
                poller.add(&accepted_socket, Event::all(event_key))?;

                let conn = Box::new(Connection::new(accepted_socket));
                connections.insert(event_key, conn);
                event_key += 1;
            }
        }
    }
}
