use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub fn write_msg(client: &mut TcpStream, msg: &str) -> std::io::Result<()> {
    let mut msg_bytes: Vec<u8> = msg.bytes().collect();

    let len = msg_bytes.len();
    let mut len_bytes = u32::try_from(len)
        .unwrap_or_default()
        .to_le_bytes()
        .into_iter()
        .collect();

    let mut bytes = Vec::with_capacity(4 + len);
    bytes.append(&mut len_bytes);
    bytes.append(&mut msg_bytes);

    client.write_all(&bytes)?;
    Ok(())
}

pub fn recieve_message(stream: &mut TcpStream) -> Result<String, String> {
    let mut size_bytes = [0; 4];
    let Ok(_) = stream.read(&mut size_bytes) else {
        return Err(String::from("could not read msg size from stream"));
    };

    let msg_size = u32::from_le_bytes(size_bytes);
    if msg_size == 0 {
        let _ = stream.shutdown(Shutdown::Both);
        return Err(String::from("Shutdown stream"));
    }

    let mut msg_bytes: Vec<u8> = Vec::with_capacity(msg_size as usize);
    let mut msg_buf: Vec<u8> = vec![0; msg_size as usize];
    let mut total = 0;

    while total < msg_size {
        let Ok(n) = stream.read(&mut msg_buf) else {
            return Err(String::from("could not read msg from stream"));
        };
        total += u32::try_from(n).unwrap_or_default();
        msg_bytes.append(&mut msg_buf);
    }

    let Ok(message) = String::from_utf8(msg_bytes) else {
        return Err(String::from(
            "could not convert message to valid utf-8 string",
        ));
    };

    Ok(message)
}
