use anyhow::{anyhow, Result};
use alloc::vec::Vec;
use femtopb::{Message};
use std::net::{TcpStream};
use async_std::sync::Mutex;
use async_io::Async;
use async_std::channel::{Receiver, Sender};
use env_logger::Logger;
use futures::{join, AsyncReadExt};
use crate::metadata::MessageType;
use crate::api::{*};
use crate::server::ConnectionStatus;
use crate::{Command, DeviceConfig, EntityConfig, StateChange};
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

pub struct EspHomeServer<'a, 's> {
    connection: EspHomeConnection,
    device_config: &'a DeviceConfig<'a>,
    state_change_channel: Receiver<StateChange<'s>>,
    command_channel: Sender<Command>,
    entity_configs: &'a[EntityConfig<'a>],
}

impl<'a, 's> EspHomeServer<'a, 's> {
    pub fn new(
        connection: EspHomeConnection,
        device_config: &'a DeviceConfig<'a>,
        entity_configs: &'a[EntityConfig<'a>],
        state_change_channel: Receiver<StateChange<'s>>,
        command_channel: Sender<Command>,
    ) -> Self {
        if device_config.password.is_none() {
            connection.status.lock_blocking().authenticated = true;
        }

        Self {
            connection,
            device_config,
            entity_configs,
            state_change_channel,
            command_channel,
        }
    }

    async fn read_frame(&self) -> Result<(MessageType, Vec<u8>)> {
        self.connection.read_frame().await
    }

    async fn send<'m, M: Message<'m>>(&self, msg_type: MessageType, message: &'m M) -> anyhow::Result<()> {
        log::info!("Sending message of type {:?}", msg_type);
        self.connection.send(msg_type, message).await
    }

    pub async fn run(&mut self) -> Result<()> {
        let _ = join!(self.run_channel_loop(), self.run_socket_loop());

        Ok(())
    }

    pub async fn run_channel_loop(&self) -> Result<()> {
        loop {
            let state_change = self.state_change_channel.recv().await?;

            let status = self.connection.status.lock().await;
            if !status.subscribed_to_states {
                log::warn!("Not subscribed to states, skipping state change");
                continue;
            }
            
            log::info!("State change received");

            match state_change {
                StateChange::BinarySensorChange(state) => {
                    let request: BinarySensorStateResponse = state.into();
                    self.send(MessageType::BinarySensorStateResponse, &request).await?;
                }
                StateChange::CoverChange(state) => {
                    let request: CoverStateResponse = state.into();
                    self.send(MessageType::CoverStateResponse, &request).await?;
                }
                StateChange::SwitchStateChange(state) => {
                    let request: SwitchStateResponse = state.into();
                    self.send(MessageType::SwitchStateResponse, &request).await?;
                }
                StateChange::FanStateChange(state) => {
                    todo!()
                }
                StateChange::LightStateChange(state) => {
                    todo!();
                }
                StateChange::SensorStateChange(state) => {
                    todo!();
                }
                StateChange::TextSensorStateChange(state) => {
                    todo!();
                }
            }
        }
    }

    pub async fn run_socket_loop(&self) -> Result<()> {
        loop {
            let (message_type, frame_data) = self.connection.read_frame().await?;
            self.handle_message(message_type, frame_data).await?;
        }
    }

    async fn validate_status(&self, message_type: MessageType) -> Result<()> {
        let status = self.connection.status.lock().await;
        if message_type.needs_authentication() && !status.authenticated {
            return Err(anyhow!("Not authenticated"));
        }
        if message_type.needs_setup_connection() && !status.setup_complete {
            return Err(anyhow!("Connection not set up"));
        }
        Ok(())
    }

    /// Handle an incoming message
    async fn handle_message(&self, msg_type: MessageType, data: Vec<u8>) -> Result<()> {
        log::info!("Got message type: {:?}", msg_type);

        if let Err(e) = self.validate_status(msg_type).await {
            log::error!("Validation error: {}", e);
            return Err(e);
        }

        match msg_type {
            // Messages allowed before setup/auth
            MessageType::HelloRequest => {
                let _ = HelloRequest::decode(&data).map_err(|e| anyhow!(e))?;
                let response = HelloResponse {
                    api_version_major: 1,
                    api_version_minor: 5,
                    name: self.device_config.name,
                    server_info: "esphome-device-rs".into(),
                    ..Default::default()
                };
                self.send(MessageType::HelloResponse, &response).await?;

                let mut status = self.connection.status.lock().await;
                status.setup_complete = true;
            }

            MessageType::ConnectRequest => {
                let request = ConnectRequest::decode(&data).map_err(|e| anyhow!(e))?;
                
                let invalid_password = if let Some(password) = self.device_config.password { 
                    request.password != password
                } else {
                    false
                };
                
                let response = ConnectResponse {
                    invalid_password,
                    ..Default::default()
                };
                self.send(MessageType::ConnectResponse, &response).await?;

                // Check if authentication succeeded
                if !response.invalid_password {
                    let mut status = self.connection.status.lock().await;
                    status.authenticated = true;
                }
            }
            
            MessageType::DisconnectRequest => {
                let _ = DisconnectRequest::decode(&data).map_err(|e| anyhow!(e))?;
                self.send(MessageType::DisconnectResponse, &DisconnectResponse::default()).await?;
                
                log::info!("Client requested disconnect");
                // Close the connection
            }
            
            MessageType::DeviceInfoRequest => {
                let _ = DeviceInfoRequest::decode(&data).map_err(|e| anyhow!(e))?;
                let response = DeviceInfoResponse {
                    name: self.device_config.name,
                    mac_address: self.device_config.mac_address,
                    esphome_version: self.device_config.esphome_version,
                    compilation_time: self.device_config.compilation_time,
                    model: self.device_config.model,
                    has_deep_sleep: self.device_config.has_deep_sleep,
                    project_name: self.device_config.project_name,
                    project_version: self.device_config.project_version,
                    webserver_port: self.device_config.webserver_port,
                    bluetooth_mac_address: self.device_config.bluetooth_mac_address,
                    manufacturer: self.device_config.manufacturer,
                    friendly_name: self.device_config.friendly_name,
                    suggested_area: self.device_config.suggested_area,
                    ..Default::default()
                };
                self.send(MessageType::DeviceInfoResponse, &response).await?;
            }

            MessageType::PingRequest => {
                let _ = PingRequest::decode(&data).map_err(|e| anyhow!(e))?;
                self.send(MessageType::PingResponse, &PingResponse::default()).await?;
            }
            
            MessageType::ListEntitiesRequest => {
                let _ = ListEntitiesRequest::decode(&data).map_err(|e| anyhow!(e))?;
                for entity in self.entity_configs.iter() {
                    match entity {
                        EntityConfig::BinarySensor(config) => {
                            let response: ListEntitiesBinarySensorResponse = config.into();
                            self.send(MessageType::ListEntitiesBinarySensorResponse, &response).await?;
                        },
                        EntityConfig::Switch(config) => {
                            let response: ListEntitiesSwitchResponse = config.into();
                            self.send(MessageType::ListEntitiesSwitchResponse, &response).await?;
                        },
                    }
                }
                self.send(MessageType::ListEntitiesDoneResponse, &ListEntitiesDoneResponse::default()).await?;
            }
            
            MessageType::SubscribeStatesRequest => {
                let _ = SubscribeStatesRequest::decode(&data).map_err(|e| anyhow!(e))?;
                let mut status = self.connection.status.lock().await;
                status.subscribed_to_states = true;
            }
            
            MessageType::SubscribeLogsRequest => {
                let _ = SubscribeLogsRequest::decode(&data).map_err(|e| anyhow!(e))?;
                let mut status = self.connection.status.lock().await;
                status.subscribed_to_logs = true;
            }

            MessageType::SwitchCommandRequest => {
                let request = SwitchCommandRequest::decode(&data).map_err(|e| anyhow!(e))?;
                let command = Command::SwitchCommand(request.into());
                self.command_channel.send(command).await?;
            }
            
            MessageType::CoverCommandRequest | 
            MessageType::FanCommandRequest |
            MessageType::LightCommandRequest |
            MessageType::ClimateCommandRequest |
            MessageType::NumberCommandRequest |
            MessageType::SelectCommandRequest |
            MessageType::SirenCommandRequest |
            MessageType::LockCommandRequest |
            MessageType::ButtonCommandRequest |
            MessageType::AlarmControlPanelCommandRequest |
            MessageType::TextCommandRequest |
            MessageType::DateCommandRequest |
            MessageType::TimeCommandRequest |
            MessageType::ValveCommandRequest |
            MessageType::DateTimeCommandRequest => {
                log::warn!("Command of type {:?} is not supported yet", msg_type);
            }
            
            MessageType::SubscribeHomeassistantServicesRequest | 
            MessageType::SubscribeHomeAssistantStatesRequest |
            MessageType::ExecuteServiceRequest |
            MessageType::CameraImageRequest | 
            MessageType::MediaPlayerCommandRequest |
            MessageType::SubscribeBluetoothLEAdvertisementsRequest | 
            MessageType::BluetoothDeviceRequest |
            MessageType::BluetoothGATTGetServicesRequest |
            MessageType::BluetoothGATTReadRequest |
            MessageType::BluetoothGATTWriteRequest |
            MessageType::BluetoothGATTReadDescriptorRequest |
            MessageType::BluetoothGATTWriteDescriptorRequest |
            MessageType::BluetoothGATTNotifyRequest |
            MessageType::SubscribeBluetoothConnectionsFreeRequest |
            MessageType::UnsubscribeBluetoothLEAdvertisementsRequest |
            MessageType::SubscribeVoiceAssistantRequest |
            MessageType::VoiceAssistantResponse |
            MessageType::VoiceAssistantEventResponse |
            MessageType::VoiceAssistantTimerEventResponse |
            MessageType::VoiceAssistantAnnounceRequest |
            MessageType::VoiceAssistantConfigurationRequest |
            MessageType::VoiceAssistantSetConfiguration |
            MessageType::BluetoothScannerSetModeRequest |
            MessageType::VoiceAssistantAudio |
            MessageType::UpdateCommandRequest => {
                log::error!("Message of type {:?} is not supported, and isn't planned for support", msg_type);
            }
            
            MessageType::NoiseEncryptionSetKeyRequest | 
            MessageType::GetTimeRequest => {
                log::warn!("Support for message of type {:?} is not implemented yet", msg_type);
            }

            _ => {
                return Err(anyhow!("Unsupported message type: {:?}", msg_type));
            }
        }

        Ok(())
    }
}