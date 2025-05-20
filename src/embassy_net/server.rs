use anyhow::{Result, anyhow, format_err};
use alloc::vec::Vec;
use femtopb::{Message};
use embassy_net::tcp::{TcpReader, TcpSocket, TcpWriter};
use crate::metadata::MessageType;
use crate::api::{*};
use crate::ApiConnection;
use crate::embassy_net::frame_reader::EspHomeFrameReader;

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
pub struct EspHomeConnection<'a> {
    reader: EspHomeFrameReader<'a>,
    writer: &'a mut TcpWriter<'a>,
    authenticated: bool,
    setup_complete: bool,
}

impl<'a> EspHomeConnection<'a> {
    pub fn new(reader: &'a mut TcpReader<'a>, writer: &'a mut TcpWriter<'a>) -> Self {
        let reader = EspHomeFrameReader::new(reader, 4096);

        Self {
            reader,
            writer,
            authenticated: false,
            setup_complete: false,
        }
    }

    /// Send a message to the client
    async fn send<'m, M: Message<'m>>(&mut self, msg_type: MessageType, message: &'m M) -> Result<()> {
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

    /// Handle an incoming message
    async fn handle_message<'b, T: ApiConnection>(&mut self, msg_type: MessageType, data: Vec<u8>, service: &'b mut T) -> Result<()> {
        match msg_type {
            // Messages allowed before setup/auth
            MessageType::HelloRequest => {
                let request = HelloRequest::decode(&data).map_err(|e| anyhow!(e))?;
                let response = service.hello(request).await?;
                self.send(MessageType::HelloResponse, &response).await?;

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
                let request = DisconnectRequest::decode(&data).map_err(|e| anyhow!(e))?;
                let response = service.disconnect(request).await?;
                self.send(MessageType::DisconnectResponse, &response).await?;
                return Err(anyhow!("Client requested disconnect"));
            }

            MessageType::PingRequest => {
                let request = PingRequest::decode(&data).map_err(|e| anyhow!(e))?;
                let response = service.ping(request).await?;
                self.send(MessageType::PingResponse, &response).await?;
            }

            MessageType::DeviceInfoRequest => {
                let request = DeviceInfoRequest::decode(&data).map_err(|e| anyhow!(e))?;
                let response = service.device_info(request).await?;
                self.send(MessageType::DeviceInfoResponse, &response).await?;
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
                    _ => {
                        // Unsupported message type
                        unimplemented!("Message type not implemented: {:?}", msg_type);
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

    /// Start the server
    pub async fn run(&mut self, socket: &mut TcpSocket<'_>, port: u16) -> Result<()> {
        loop {
            socket.accept(port).await.map_err(|_| format_err!("Socket accept error"))?;

            // Spawn a new task to handle this connection
            #[cfg(feature = "embassy-executor")]
            embassy_executor::spawn(async move {
                let mut conn_handler = EspHomeConnection::new(connection, service);
                if let Err(e) = conn_handler.handle().await {
                    // Log error
                    log::error!("Connection error: {:?}", e);
                }
            });

            // Alternative if you're not using Embassy's executor
            #[cfg(not(feature = "embassy-executor"))]
            {
                let (mut reader, mut writer) = socket.split();

                // Handle in the current task (blocking for new connections)
                let mut conn_handler = EspHomeConnection::new(&mut reader, &mut writer);
                if let Err(e) = conn_handler.handle(&mut self.service).await {
                    // Log error
                    log::error!("Connection error: {:?}", e);
                }
            }
        }
    }
}