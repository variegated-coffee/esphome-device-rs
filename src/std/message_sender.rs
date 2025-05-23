use std::net::TcpStream;
use anyhow::anyhow;
use async_io::Async;
use femtopb::Message;
use futures::AsyncWriteExt;
use futures::io::WriteHalf;
use crate::metadata::MessageType;


/// Helper function to write a varint to a buffer
pub fn write_varint(buffer: &mut Vec<u8>, value: u64) {
    let mut val = value;
    loop {
        let mut byte = (val & 0x7f) as u8;
        val >>= 7;
        if val != 0 {
            byte |= 0x80;
        }
        buffer.push(byte);
        if val == 0 {
            break;
        }
    }
}

/// Helper function to serialize a message in ESPHome format
pub fn serialize_message<'a, T: Message<'a>>(msg_type: MessageType, message: &T) -> anyhow::Result<Vec<u8>> {
    let mut buffer = Vec::new();

    // 1. Zero byte
    buffer.push(0);

    // Calculate message size first to create a buffer of the right size
    let encoded_size = message.encoded_len();
    let mut message_data = Vec::with_capacity(encoded_size);
    message_data.resize(encoded_size, 0);

    // Encode the message with femtopb
    message.encode(&mut message_data.as_mut_slice()).map_err(|e| anyhow!(e))?;

    // 2. Write size varint
    write_varint(&mut buffer, encoded_size as u64);

    // 3. Write type varint
    write_varint(&mut buffer, msg_type as u64);

    // 4. Write message data
    buffer.extend_from_slice(&message_data);

    Ok(buffer)
}

pub struct MessageSender {
    writer: WriteHalf<Async<TcpStream>>,
}

impl MessageSender {
    pub fn new(writer: WriteHalf<Async<TcpStream>>) -> Self {
        Self { writer }
    }

    /// Send a message to the client
    pub(crate) async fn send<'m, M: Message<'m>>(&mut self, msg_type: MessageType, message: &'m M) -> anyhow::Result<()> {
        let data = serialize_message(msg_type, message)?;

        let mut sent = 0;
        while sent < data.len() {
            match self.writer.write(&data[sent..]).await {
                Ok(0) => return Err(anyhow!("Connection closed during send")),
                Ok(n) => sent += n,
                Err(e) => return Err(anyhow!("Connection write error: {:?}", e)),
            }
        }

        Ok(())
    }
}