use anyhow::{Result, anyhow, format_err};
use alloc::vec::Vec;
use femtopb::{Message};
use std::net::{TcpStream, TcpListener};
use async_io::Async;
use futures::{AsyncReadExt, AsyncWriteExt, FutureExt};
use futures::io::WriteHalf;
use crate::metadata::MessageType;
use crate::api::{*};
use crate::ApiConnection;
use crate::std::frame_reader::EspHomeFrameReader;

/// Macro to handle request-response pattern
macro_rules! handle_request_response {
    ($self:expr, $service:expr, $data:expr, $request_type:ty, $service_method:ident, $response_type:expr) => {
        let request = <$request_type>::decode(&$data).map_err(|e| anyhow!(e))?;
        let response = $service.$service_method(request).await?;
        $self.send($response_type, &response).await?
    };
}

macro_rules! handle_request_without_response {
    ($self:expr, $service:expr, $data:expr, $request_type:ty, $service_method:ident) => {
        let request = <$request_type>::decode(&$data).map_err(|e| anyhow!(e))?;
        let _ = $service.$service_method(request).await?;
    };
}

/// Helper function to write a varint to a buffer
fn write_varint(buffer: &mut Vec<u8>, value: u64) {
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
fn serialize_message<'a, T: Message<'a>>(msg_type: MessageType, message: &T) -> Result<Vec<u8>> {
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

/// ESPHome connection handler
pub struct EspHomeConnection {
    reader: EspHomeFrameReader,
    stream: WriteHalf<Async<TcpStream>>,
    authenticated: bool,
    setup_complete: bool,
}

impl EspHomeConnection {
    pub fn new(stream: Async<TcpStream>) -> Self {
        // Create frame reader with the stream
        let (reader, writer) = stream.split();
        let reader = EspHomeFrameReader::new(reader, 4096);

        Self {
            reader,
            stream: writer,
            authenticated: false,
            setup_complete: false,
        }
    }

    /// Send a message to the client
    async fn send<'m, M: Message<'m>>(&mut self, msg_type: MessageType, message: &'m M) -> Result<()> {
        let data = serialize_message(msg_type, message)?;

        let mut sent = 0;
        while sent < data.len() {
            match self.stream.write(&data[sent..]).await {
                Ok(0) => return Err(anyhow!("Connection closed during send")),
                Ok(n) => sent += n,
                Err(e) => return Err(anyhow!("Connection write error: {:?}", e)),
            }
        }

        Ok(())
    }

    /// Handle an incoming message
    async fn handle_message<'b, T: ApiConnection>(&mut self, msg_type: MessageType, data: Vec<u8>, service: &'b mut T) -> Result<()> {
        match msg_type {
            // Messages allowed before setup/auth
            MessageType::HelloRequest => {
                handle_request_response!(self, service, data, HelloRequest, hello, MessageType::HelloResponse);
                self.setup_complete = true;
            }

            MessageType::ConnectRequest => {
                if !self.setup_complete {
                    return Err(anyhow!("Connection not set up"));
                }

                let request = ConnectRequest::decode(&data).map_err(|e| anyhow!(e))?;
                let response_future = service.connect(request);
                let response = response_future.await?;
                self.send(MessageType::ConnectResponse, &response).await?;

                // Check if authentication succeeded
                if !response.invalid_password {
                    self.authenticated = true;
                }
            }

            MessageType::DisconnectRequest => {
                handle_request_response!(self, service, data, DisconnectRequest, disconnect, MessageType::DisconnectResponse);
                return Err(anyhow!("Client requested disconnect"));
            }

            MessageType::PingRequest => {
                handle_request_response!(self, service, data, PingRequest, ping, MessageType::PingResponse);
            }

            MessageType::DeviceInfoRequest => {
                handle_request_response!(self, service, data, DeviceInfoRequest, device_info, MessageType::DeviceInfoResponse);
            }

            // Messages that need authentication
            _ => {
                // Check if message needs authentication
                if msg_type.needs_authentication() && !self.authenticated {
                    return Err(anyhow!("Not authenticated"));
                }

                // Check if message needs setup
                if msg_type.needs_setup_connection() && !self.setup_complete {
                    return Err(anyhow!("Connection not set up"));
                }

                // Handle other message types based on your ApiConnection implementation
                match msg_type {
                    MessageType::SubscribeStatesRequest => {
                        handle_request_without_response!(self, service, data, SubscribeStatesRequest, subscribe_states);
                    }
                    MessageType::SubscribeLogsRequest => {
                        handle_request_response!(self, service, data, SubscribeLogsRequest, subscribe_logs, MessageType::SubscribeLogsResponse);
                    }
                    MessageType::SubscribeHomeassistantServicesRequest => {
                        handle_request_without_response!(self, service, data, SubscribeHomeassistantServicesRequest, subscribe_homeassistant_services);
                    }
                    MessageType::SubscribeHomeAssistantStatesRequest => {
                        handle_request_without_response!(self, service, data, SubscribeHomeAssistantStatesRequest, subscribe_home_assistant_states);
                    }
                    _ => {
                        return Err(anyhow!("Unsupported message type: {:?}", msg_type));
                    }
                }
            }
        }

        Ok(())
    }

    /// Main connection handling loop
    pub async fn handle<'b, T: ApiConnection>(&mut self, service: &'b mut T) -> Result<()> {
        // Process messages until error or disconnect
        loop {
            match self.reader.read_frame().await {
                Ok((msg_type, data)) => {
                    if let Err(e) = self.handle_message(msg_type, data, service).await {
                        return Err(e);
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
}

/// ESPHome server that listens for connections
pub struct EspHomeServer<T: ApiConnection + Clone> {
    service: T,
}

impl<T: ApiConnection + Clone + Send + 'static> EspHomeServer<T> {
    pub fn new(service: T) -> Self {
        Self { service }
    }
    
    pub async fn handle(&mut self, stream: Async<TcpStream>) -> Result<()> {
        // Create a new connection handler
        let mut conn_handler = EspHomeConnection::new(stream);
        
        // Handle the connection with the service
        conn_handler.handle(&mut self.service).await
    }
}
