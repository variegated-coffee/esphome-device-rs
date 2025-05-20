use embassy_net::tcp::{TcpReader};
use anyhow::{Result, anyhow};
use alloc::vec::Vec;
use crate::metadata::MessageType;

pub struct EspHomeFrameReader<'a> {
    connection: &'a mut TcpReader<'a>,
    buffer: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
}

impl<'a> EspHomeFrameReader<'a> {
    pub fn new(connection: &'a mut TcpReader<'a>, buffer_size: usize) -> Self {
        let mut buffer = Vec::with_capacity(buffer_size);
        buffer.resize(buffer_size, 0);

        Self {
            connection,
            buffer,
            read_pos: 0,
            write_pos: 0,
        }
    }

    async fn fill_buffer(&mut self) -> Result<usize> {
        // If buffer is full, reset positions to make space
        if self.write_pos >= self.buffer.len() {
            // If we've consumed everything, reset completely
            if self.read_pos >= self.write_pos {
                self.read_pos = 0;
                self.write_pos = 0;
            } else {
                // Shift remaining data to the start
                for i in 0..(self.write_pos - self.read_pos) {
                    self.buffer[i] = self.buffer[self.read_pos + i];
                }
                self.write_pos -= self.read_pos;
                self.read_pos = 0;
            }
        }
        
        // Read from connection
        match self.connection.read(&mut self.buffer[self.write_pos..]).await {
            Ok(0) => return Err(anyhow!("Connection closed")),
            Ok(n) => {
                self.write_pos += n;
                Ok(n)
            },
            Err(e) => Err(anyhow!("Connection read error: {:?}", e)),
        }
    }

    async fn read_byte(&mut self) -> Result<u8> {
        while self.read_pos >= self.write_pos {
            self.fill_buffer().await?;
        }

        let byte = self.buffer[self.read_pos];
        self.read_pos += 1;
        Ok(byte)
    }

    async fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        let mut read = 0;

        while read < buf.len() {
            if self.read_pos >= self.write_pos {
                self.fill_buffer().await?;
            }

            let to_copy = core::cmp::min(buf.len() - read, self.write_pos - self.read_pos);
            buf[read..read+to_copy].copy_from_slice(&self.buffer[self.read_pos..self.read_pos+to_copy]);

            self.read_pos += to_copy;
            read += to_copy;
        }

        Ok(())
    }

    async fn read_varint(&mut self) -> Result<u64> {
        let mut result: u64 = 0;
        let mut shift: u32 = 0;

        loop {
            let byte = self.read_byte().await?;

            result |= ((byte & 0x7f) as u64) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
            if shift > 63 {
                return Err(anyhow!("Varint too long"));
            }
        }

        Ok(result)
    }

    pub async fn read_frame(&mut self) -> Result<(MessageType, Vec<u8>)> {
        // 1. Read the zero byte
        let zero_byte = self.read_byte().await?;
        if zero_byte != 0 {
            return Err(anyhow!("Expected zero byte, got {}", zero_byte));
        }

        // 2. Read the size varint
        let size = self.read_varint().await? as usize;

        // 3. Read the type varint
        let type_id = self.read_varint().await? as u8;
        let message_type = MessageType::from_u8(type_id)
            .ok_or_else(|| anyhow!("Unknown message type: {}", type_id))?;

        // 4. Read the message payload
        let mut payload = Vec::with_capacity(size);
        payload.resize(size, 0);
        self.read_exact(&mut payload).await?;

        Ok((message_type, payload))
    }
}