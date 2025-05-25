use anyhow::{Result};
use alloc::vec::Vec;
use femtopb::{Message};
use std::net::{TcpStream};
use async_std::sync::Mutex;
use async_io::Async;
use futures::{AsyncReadExt};
use crate::metadata::MessageType;
use crate::server::ConnectionStatus;
use crate::std::frame_reader::EspHomeFrameReader;
use crate::std::message_sender::MessageSender;

/// ESPHome connection handler
pub struct EspHomeConnection {
    reader: Mutex<EspHomeFrameReader>,
    writer: Mutex<MessageSender>,
    pub status: Mutex<ConnectionStatus>,
}


impl EspHomeConnection {
    pub fn new(stream: Async<TcpStream>) -> Self {
        let (reader, writer) = stream.split();
        let reader = Mutex::new(EspHomeFrameReader::new(reader, 4096));
        let writer = Mutex::new(MessageSender::new(writer));

        Self {
            reader,
            writer,
            status: Mutex::new(ConnectionStatus::default()),
        }
    }

    pub async fn send<'m, M: Message<'m>>(&self, msg_type: MessageType, message: &'m M) -> Result<()> {
        self.writer.lock().await.send(msg_type, message).await
    }

    pub async fn read_frame(&self) -> Result<(MessageType, Vec<u8>)> {
        self.reader.lock().await.read_frame().await 
    }
}
