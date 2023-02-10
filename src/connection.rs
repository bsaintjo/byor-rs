use std::{
    error::Error,
    io::{ErrorKind, Read},
    os::fd::AsRawFd,
};

use byteorder::{NetworkEndian, ReadBytesExt};
use socket2::Socket;

use crate::K_MAX_MSG;

#[derive(PartialEq, Clone, Copy)]
pub enum State {
    Request,
    Response,
    End,
}

pub struct Connection {
    pub socket: Socket,
    pub state: State,
    read_size: usize,
    read_buffer: [u8; 4 + K_MAX_MSG],
    write_buffer_size: usize,
    write_buffer_sent: usize,
    write_buffer: [u8; 4 + K_MAX_MSG],
}

impl Connection {
    pub fn new(socket: Socket) -> Self {
        Connection {
            socket,
            state: State::Request,
            read_size: 0,
            read_buffer: [0; 4 + K_MAX_MSG],
            write_buffer_size: 0,
            write_buffer_sent: 0,
            write_buffer: [0; 4 + K_MAX_MSG],
        }
    }

    pub fn fd_u64(&self) -> Result<u64, std::num::TryFromIntError> {
        self.socket.as_raw_fd().try_into()
    }

    pub fn conn_io(&mut self) -> Result<(), Box<dyn Error>> {
        match self.state {
            State::Request => self.request()?,
            State::Response => self.response()?,
            State::End => unreachable!("End state"),
        }
        Ok(())
    }

    pub fn request(&mut self) -> Result<(), Box<dyn Error>> {
        while self.try_fill_buffer()? {}
        Ok(())
    }

    pub fn try_fill_buffer(&mut self) -> Result<bool, Box<dyn Error>> {
        let res = loop {
            match self.socket.read(&mut self.read_buffer[self.read_size..]) {
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                res => break res,
            }
        };
        match res {
            Ok(0) => {
                if self.read_size > 0 {
                    log::warn!("Unexpected EOF");
                } else {
                    log::info!("EOF");
                }

                self.state = State::End;
                Ok(false)
            }
            Ok(n) => {
                self.read_size += n;
                while self.try_one_request()? {}
                Ok(self.state == State::Request)
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => Ok(false),
            Err(e) => {
                log::error!("Read failed with error {e}");
                self.state = State::End;
                Ok(false)
            }
        }
    }

    pub fn try_one_request(&mut self) -> Result<bool, Box<dyn Error>> {
        if self.read_size < 4 {
            log::info!("Not enough data in buffer, retrying next iteration");
            return Ok(false);
        }
        let len = self.socket.read_u32::<NetworkEndian>()? as usize;
        if len > K_MAX_MSG {
            log::error!("Message length too long");
            self.state = State::End;
            return Ok(false);
        }

        if (4 + len) > self.read_size {
            log::info!("Still not enough data, retrying later");
            return Ok(false);
        }

        println!(
            "client says {}",
            std::str::from_utf8(&self.read_buffer[4..4 + len])?
        );
        todo!()
    }

    pub fn response(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
