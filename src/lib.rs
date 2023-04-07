pub mod mio;
pub mod connection;
pub mod polling;
// pub mod epoll;
pub mod startup;

use std::io::{Read, Write};

pub const K_MAX_MSG: usize = 4096;

pub fn read_msg<R>(mut src: R) -> Result<String, Box<dyn std::error::Error>>
where
    R: Read,
{
    log::info!("Reading message");
    let mut read_buffer = [0u8; 4 + K_MAX_MSG + 1];
    src.read_exact(&mut read_buffer[..4])?;
    let len: [u8; 4] = read_buffer[..4].try_into().unwrap();
    let len: u32 = u32::from_be_bytes(len);
    log::info!("Read len: {len}");
    if len > (K_MAX_MSG as u32) {
        return Err(format!("len {len} is longer than K_MAX_MSG: {K_MAX_MSG}").into());
    }
    src.read_exact(&mut read_buffer[4 .. 4 + (len as usize)])?;
    let client_response = std::str::from_utf8(&read_buffer[4..4 + (len as usize)])?;
    Ok(client_response.to_string())
}

pub fn send_msg<W, T>(mut dst: W, text: T) -> Result<(), Box<dyn std::error::Error>>
where
    W: Write,
    T: AsRef<[u8]>,
{
    log::info!("Sending message");
    let text = text.as_ref();
    let len = text.len();
    if len > K_MAX_MSG {
        return Err("Failed to ".into());
    }
    log::info!("text len {len}");

    let len_bytes: [u8; 4] = (len as u32).to_be_bytes();
    let mut write_buffer = [0u8; 4 + K_MAX_MSG];
    (&mut write_buffer[..4]).write_all(&len_bytes)?;
    (&mut write_buffer[4..]).write_all(text)?;
    log::debug!("write buffer: {write_buffer:?}");
    dst.write_all(&write_buffer[..4 + len])?;
    log::info!("Successfully sent message");
    Ok(())
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_read_msg() {
        const TEXT: &[u8] = b"hello";
        let mut buf = Cursor::new(Vec::new());
        send_msg(&mut buf, TEXT).unwrap();
        buf.set_position(0);
        let msg = read_msg(&mut buf).unwrap();
        assert_eq!("hello".to_string(), msg);
    }
}
