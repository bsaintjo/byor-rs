use std::io::Read;

pub const K_MAX_MSG: usize = 4096;

pub fn read_msg<R>(mut src: R) -> Result<String, Box<dyn std::error::Error>>
where
    R: Read,
{
    let mut read_buffer = [0u8; 4 + K_MAX_MSG + 1];
    src.read_exact(&mut read_buffer[..4])?;
    let len: [u8; 4] = read_buffer[..4].try_into().unwrap();
    let len: u32 = u32::from_be_bytes(len);
    if len > (K_MAX_MSG as u32) {
        return Err(format!("len {len} is longer than K_MAX_MSG: {K_MAX_MSG}").into());
    }
    let msg_span = 4..4 + (len as usize);
    src.read_exact(&mut read_buffer[msg_span.clone()])?;
    let client_response = std::str::from_utf8(&read_buffer[msg_span])?;
    Ok(client_response.to_string())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use byteorder::{WriteBytesExt, NetworkEndian};

    use super::*;

    #[test]
    fn test_read_msg() {
        const text: &[u8] = b"hello";
        const len: [u8; 4] = (text.len() as u32).to_le_bytes();
        let mut buf = [0u8; 9];
        (&mut buf[..4]).write_u32::<NetworkEndian>(text.len() as u32).unwrap();
        (&mut buf[4..]).write_all(text).unwrap();
        let msg = read_msg(buf.as_ref()).unwrap();
        assert_eq!("hello".to_string(), msg);
    }

    #[test]
    fn test_len_conversion() {
        let len = 10u32;
        let res = len.to_le_bytes();
        let res: u32 = unsafe { std::mem::transmute(res) };
        assert_eq!(len, res);
    }
}
