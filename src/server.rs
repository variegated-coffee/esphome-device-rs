use crate::api::{AlarmControlPanelCommandRequest, ButtonCommandRequest, ClimateCommandRequest, CoverCommandRequest, DateCommandRequest, DateTimeCommandRequest, FanCommandRequest, LightCommandRequest, LockCommandRequest, NumberCommandRequest, SelectCommandRequest, SirenCommandRequest, TextCommandRequest, TimeCommandRequest, ValveCommandRequest};
use anyhow::anyhow;
use async_std::channel::{Receiver, Sender};
use femtopb::Message;
use futures::join;
use crate::{ClientEvent, Command, DeviceConfig, EntityConfig, StateChange};
use crate::api::{AlarmControlPanelStateResponse, BinarySensorStateResponse, ClimateStateResponse, ConnectRequest, ConnectResponse, CoverStateResponse, DateStateResponse, DateTimeStateResponse, DeviceInfoRequest, DeviceInfoResponse, DisconnectRequest, DisconnectResponse, EventResponse, FanStateResponse, HelloRequest, HelloResponse, LightStateResponse, ListEntitiesAlarmControlPanelResponse, ListEntitiesBinarySensorResponse, ListEntitiesButtonResponse, ListEntitiesClimateResponse, ListEntitiesCoverResponse, ListEntitiesDateResponse, ListEntitiesDateTimeResponse, ListEntitiesDoneResponse, ListEntitiesEventResponse, ListEntitiesFanResponse, ListEntitiesLightResponse, ListEntitiesLockResponse, ListEntitiesNumberResponse, ListEntitiesRequest, ListEntitiesSelectResponse, ListEntitiesSensorResponse, ListEntitiesSirenResponse, ListEntitiesSwitchResponse, ListEntitiesTextResponse, ListEntitiesTextSensorResponse, ListEntitiesTimeResponse, ListEntitiesValveResponse, LockStateResponse, NumberStateResponse, PingRequest, PingResponse, SelectStateResponse, SensorStateResponse, SirenStateResponse, SubscribeLogsRequest, SubscribeStatesRequest, SwitchCommandRequest, SwitchStateResponse, TextSensorStateResponse, TextStateResponse, TimeStateResponse, ValveStateResponse};
use crate::metadata::MessageType;
use crate::std::server::EspHomeConnection;

#[derive(Default)]
pub struct ConnectionStatus {
    pub authenticated: bool,
    pub setup_complete: bool,
    pub subscribed_to_states: bool,
    pub subscribed_to_logs: bool,
}

pub struct EspHomeServer<'a, 's> {
    connection: EspHomeConnection,
    device_config: &'a DeviceConfig<'a>,
    state_change_channel: Receiver<StateChange<'s>>,
    client_event_channel: Sender<ClientEvent>,
    entity_configs: &'a[EntityConfig<'a>],
}

macro_rules! handle_command {
    ($command:expr, $request_type:ty) => {
        self.client_event_channel.send(
            $command(
                <$request_type>::decode(&data)
                    .map_err(|e| anyhow!(e))?
                    .into()
            )
        ).await?
    };
}

impl<'a, 's> EspHomeServer<'a, 's> {
    pub fn new(
        connection: EspHomeConnection,
        device_config: &'a DeviceConfig<'a>,
        entity_configs: &'a[EntityConfig<'a>],
        state_change_channel: Receiver<StateChange<'s>>,
        client_event_channel: Sender<ClientEvent>,
    ) -> Self {
        if device_config.password.is_none() {
            connection.status.lock_blocking().authenticated = true;
        }

        Self {
            connection,
            device_config,
            entity_configs,
            state_change_channel,
            client_event_channel,
        }
    }

    async fn read_frame(&self) -> anyhow::Result<(MessageType, Vec<u8>)> {
        self.connection.read_frame().await
    }

    async fn send<'m, M: Message<'m>>(&self, msg_type: MessageType, message: &'m M) -> anyhow::Result<()> {
        log::info!("Sending message of type {:?}", msg_type);
        self.connection.send(msg_type, message).await
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let _ = join!(self.run_channel_loop(), self.run_socket_loop());

        Ok(())
    }

    pub async fn run_channel_loop(&self) -> anyhow::Result<()> {
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
                    self.send::<BinarySensorStateResponse>(MessageType::BinarySensorStateResponse, &state.into()).await?;
                }
                StateChange::CoverChange(state) => {
                    self.send::<CoverStateResponse>(MessageType::CoverStateResponse, &state.into()).await?;
                }
                StateChange::SwitchStateChange(state) => {
                    self.send::<SwitchStateResponse>(MessageType::SwitchStateResponse, &state.into()).await?;
                }
                StateChange::FanStateChange(state) => {
                    self.send::<FanStateResponse>(MessageType::FanStateResponse, &state.into()).await?;
                }
                StateChange::LightStateChange(state) => {
                    self.send::<LightStateResponse>(MessageType::LightStateResponse, &state.into()).await?;
                }
                StateChange::SensorStateChange(state) => {
                    self.send::<SensorStateResponse>(MessageType::SensorStateResponse, &state.into()).await?;
                }
                StateChange::TextSensorStateChange(state) => {
                    self.send::<TextSensorStateResponse>(MessageType::TextSensorStateResponse, &state.into()).await?;
                },
                StateChange::NumberStateChange(state) => {
                    self.send::<NumberStateResponse>(MessageType::NumberStateResponse, &state.into()).await?;
                }
                StateChange::SelectStateChange(state) => {
                    self.send::<SelectStateResponse>(MessageType::SelectStateResponse, &state.into()).await?;
                }
                StateChange::SirenStateChange(state) => {
                    self.send::<SirenStateResponse>(MessageType::SirenStateResponse, &state.into()).await?;
                }
                StateChange::LockStateChange(state) => {
                    self.send::<LockStateResponse>(MessageType::LockStateResponse, &state.into()).await?;
                }
                StateChange::AlarmControlPanelStateChange(state) => {
                    self.send::<AlarmControlPanelStateResponse>(MessageType::AlarmControlPanelStateResponse, &state.into()).await?;
                }
                StateChange::TextStateChange(state) => {
                    self.send::<TextStateResponse>(MessageType::TextStateResponse, &state.into()).await?;
                }
                StateChange::DateStateChange(state) => {
                    self.send::<DateStateResponse>(MessageType::DateStateResponse, &state.into()).await?;
                }
                StateChange::TimeStateChange(state) => {
                    self.send::<TimeStateResponse>(MessageType::TimeStateResponse, &state.into()).await?;
                }
                StateChange::EventStateChange(state) => {
                    self.send::<EventResponse>(MessageType::EventResponse, &state.into()).await?;
                }
                StateChange::ValveStateChange(state) => {
                    self.send::<ValveStateResponse>(MessageType::ValveStateResponse, &state.into()).await?;
                }
                StateChange::DateTimeStateChange(state) => {
                    self.send::<DateTimeStateResponse>(MessageType::DateTimeStateResponse, &state.into()).await?;
                }
                StateChange::ClimateStateChange(state) => {
                    self.send::<ClimateStateResponse>(MessageType::ClimateStateResponse, &state.into()).await?;
                }
            }
        }
    }

    pub async fn run_socket_loop(&self) -> anyhow::Result<()> {
        loop {
            let (message_type, frame_data) = self.read_frame().await?;
            self.handle_message(message_type, frame_data).await?;
        }
    }

    async fn validate_status(&self, message_type: MessageType) -> anyhow::Result<()> {
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
    async fn handle_message(&self, msg_type: MessageType, data: Vec<u8>) -> anyhow::Result<()> {
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
                            self.send::<ListEntitiesBinarySensorResponse>(MessageType::ListEntitiesBinarySensorResponse, &config.into()).await?;
                        },
                        EntityConfig::Switch(config) => {
                            self.send::<ListEntitiesSwitchResponse>(MessageType::ListEntitiesSwitchResponse, &config.into()).await?;
                        },
                        EntityConfig::Sensor(config) => {
                            self.send::<ListEntitiesSensorResponse>(MessageType::ListEntitiesSensorResponse, &config.into()).await?;
                        },
                        EntityConfig::TextSensor(config) => {
                            self.send::<ListEntitiesTextSensorResponse>(MessageType::ListEntitiesTextSensorResponse, &config.into()).await?;
                        },
                        EntityConfig::Cover(config) => {
                            self.send::<ListEntitiesCoverResponse>(MessageType::ListEntitiesCoverResponse, &config.into()).await?;
                        },
                        EntityConfig::Fan(config) => {
                            self.send::<ListEntitiesFanResponse>(MessageType::ListEntitiesFanResponse, &config.into()).await?;
                        },
                        EntityConfig::Light(config) => {
                            self.send::<ListEntitiesLightResponse>(MessageType::ListEntitiesLightResponse, &config.into()).await?;
                        },
                        EntityConfig::Climate(config) => {
                            self.send::<ListEntitiesClimateResponse>(MessageType::ListEntitiesClimateResponse, &config.into()).await?;
                        },
                        EntityConfig::Number(config) => {
                            self.send::<ListEntitiesNumberResponse>(MessageType::ListEntitiesNumberResponse, &config.into()).await?;
                        },
                        EntityConfig::Select(config) => {
                            self.send::<ListEntitiesSelectResponse>(MessageType::ListEntitiesSelectResponse, &config.into()).await?;
                        },
                        EntityConfig::Siren(config) => {
                            self.send::<ListEntitiesSirenResponse>(MessageType::ListEntitiesSirenResponse, &config.into()).await?;
                        },
                        EntityConfig::Lock(config) => {
                            self.send::<ListEntitiesLockResponse>(MessageType::ListEntitiesLockResponse, &config.into()).await?;
                        },
                        EntityConfig::Button(config) => {
                            self.send::<ListEntitiesButtonResponse>(MessageType::ListEntitiesButtonResponse, &config.into()).await?;
                        },
                        EntityConfig::AlarmControlPanel(config) => {
                            self.send::<ListEntitiesAlarmControlPanelResponse>(MessageType::ListEntitiesAlarmControlPanelResponse, &config.into()).await?;
                        },
                        EntityConfig::Text(config) => {
                            self.send::<ListEntitiesTextResponse>(MessageType::ListEntitiesTextResponse, &config.into()).await?;
                        },
                        EntityConfig::Date(config) => {
                            self.send::<ListEntitiesDateResponse>(MessageType::ListEntitiesDateResponse, &config.into()).await?;
                        },
                        EntityConfig::Time(config) => {
                            self.send::<ListEntitiesTimeResponse>(MessageType::ListEntitiesTimeResponse, &config.into()).await?;
                        },
                        EntityConfig::Event(config) => {
                            self.send::<ListEntitiesEventResponse>(MessageType::ListEntitiesEventResponse, &config.into()).await?;
                        },
                        EntityConfig::Valve(config) => {
                            self.send::<ListEntitiesValveResponse>(MessageType::ListEntitiesValveResponse, &config.into()).await?;
                        },
                        EntityConfig::DateTime(config) => {
                            self.send::<ListEntitiesDateTimeResponse>(MessageType::ListEntitiesDateTimeResponse, &config.into()).await?;
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

            MessageType::SwitchCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::SwitchCommand(SwitchCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::CoverCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::CoverCommand(CoverCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::FanCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::FanCommand(FanCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::LightCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::LightCommand(LightCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::ClimateCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::ClimateCommand(ClimateCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::NumberCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::NumberCommand(NumberCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::SelectCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::SelectCommand(SelectCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::SirenCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::SirenCommand(SirenCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::LockCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::LockCommand(LockCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::ButtonCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::ButtonCommand(ButtonCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::AlarmControlPanelCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::AlarmControlPanelCommand(AlarmControlPanelCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::TextCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::TextCommand(TextCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::DateCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::DateCommand(DateCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::TimeCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::TimeCommand(TimeCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::ValveCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::ValveCommand(ValveCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

            MessageType::DateTimeCommandRequest =>
                self.client_event_channel.send(ClientEvent::CommandReceived(Command::DateTimeCommand(DateTimeCommandRequest::decode(&data).map_err(|e| anyhow!(e))?.into()))).await?,

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