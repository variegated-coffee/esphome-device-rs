use anyhow::anyhow;
use femtopb::Message;
use crate::{ClientEvent, Command, DeviceConfig, EntityConfig, StateChange};
use crate::api::{AlarmControlPanelStateResponse, BinarySensorStateResponse, ClimateStateResponse, ConnectRequest, ConnectResponse, CoverStateResponse, DateStateResponse, DateTimeStateResponse, DeviceInfoRequest, DeviceInfoResponse, DisconnectRequest, DisconnectResponse, EventResponse, FanStateResponse, HelloRequest, HelloResponse, LightStateResponse, ListEntitiesAlarmControlPanelResponse, ListEntitiesBinarySensorResponse, ListEntitiesButtonResponse, ListEntitiesClimateResponse, ListEntitiesCoverResponse, ListEntitiesDateResponse, ListEntitiesDateTimeResponse, ListEntitiesDoneResponse, ListEntitiesEventResponse, ListEntitiesFanResponse, ListEntitiesLightResponse, ListEntitiesLockResponse, ListEntitiesNumberResponse, ListEntitiesRequest, ListEntitiesSelectResponse, ListEntitiesSensorResponse, ListEntitiesSirenResponse, ListEntitiesSwitchResponse, ListEntitiesTextResponse, ListEntitiesTextSensorResponse, ListEntitiesTimeResponse, ListEntitiesValveResponse, LockStateResponse, NumberStateResponse, PingRequest, PingResponse, SelectStateResponse, SensorStateResponse, SirenStateResponse, SubscribeLogsRequest, SubscribeStatesRequest, SwitchCommandRequest, SwitchStateResponse, TextSensorStateResponse, TextStateResponse, TimeStateResponse, ValveStateResponse};
use crate::api::{AlarmControlPanelCommandRequest, ButtonCommandRequest, ClimateCommandRequest, CoverCommandRequest, DateCommandRequest, DateTimeCommandRequest, FanCommandRequest, LightCommandRequest, LockCommandRequest, NumberCommandRequest, SelectCommandRequest, SirenCommandRequest, TextCommandRequest, TimeCommandRequest, ValveCommandRequest};
use crate::metadata::MessageType;

#[cfg(feature = "std")]
use async_std::channel::{Receiver, Sender};
#[cfg(feature = "std")]
use futures::join;
#[cfg(feature = "std")]
use crate::std::server::EspHomeConnection;

#[derive(Default)]
pub struct ConnectionStatus {
    pub authenticated: bool,
    pub setup_complete: bool,
    pub subscribed_to_states: bool,
    pub subscribed_to_logs: bool,
}

#[cfg(feature = "std")]
pub struct EspHomeServer<'a, 's> {
    connection: EspHomeConnection,
    device_config: &'a DeviceConfig<'a>,
    state_change_channel: Receiver<StateChange<'s>>,
    client_event_channel: Sender<ClientEvent>,
    entity_configs: &'a[EntityConfig<'a>],
}

#[cfg(feature = "std")]
macro_rules! handle_command_request {
    ($self:expr, $data:expr, $command_variant:ident, $request_type:ty) => {
        $self.client_event_channel.send(
            ClientEvent::CommandReceived(
                Command::$command_variant(
                    <$request_type>::decode($data)
                        .map_err(|e| anyhow!(e))?
                        .into()
                )
            )
        ).await?
    };
}

#[cfg(feature = "std")]
macro_rules! handle_state_change {
    ($self:expr, $state:expr, $response_type:ty, $msg_type:expr) => {
        $self.send::<$response_type>($msg_type, &$state.into()).await?
    };
}

#[cfg(feature = "std")]
macro_rules! handle_list_entity {
    ($self:expr, $config:expr, $response_type:ty, $msg_type:expr) => {
        $self.send::<$response_type>($msg_type, &$config.into()).await?
    };
}

#[cfg(feature = "std")]
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
        log::debug!("Sending message of type {:?}", msg_type);
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
/*            if !status.subscribed_to_states {
                log::warn!("Not subscribed to states, skipping state change");
                continue;
            }*/

            //log::info!("State change received");

            match state_change {
                StateChange::BinarySensorChange(state) => 
                    handle_state_change!(self, state, BinarySensorStateResponse, MessageType::BinarySensorStateResponse),
                
                StateChange::CoverChange(state) => 
                    handle_state_change!(self, state, CoverStateResponse, MessageType::CoverStateResponse),
                
                StateChange::SwitchStateChange(state) => 
                    handle_state_change!(self, state, SwitchStateResponse, MessageType::SwitchStateResponse),
                
                StateChange::FanStateChange(state) => 
                    handle_state_change!(self, state, FanStateResponse, MessageType::FanStateResponse),
                
                StateChange::LightStateChange(state) => 
                    handle_state_change!(self, state, LightStateResponse, MessageType::LightStateResponse),
                
                StateChange::SensorStateChange(state) => 
                    handle_state_change!(self, state, SensorStateResponse, MessageType::SensorStateResponse),
                
                StateChange::TextSensorStateChange(state) => 
                    handle_state_change!(self, state, TextSensorStateResponse, MessageType::TextSensorStateResponse),
                
                StateChange::NumberStateChange(state) => 
                    handle_state_change!(self, state, NumberStateResponse, MessageType::NumberStateResponse),
                
                StateChange::SelectStateChange(state) => 
                    handle_state_change!(self, state, SelectStateResponse, MessageType::SelectStateResponse),
                
                StateChange::SirenStateChange(state) => 
                    handle_state_change!(self, state, SirenStateResponse, MessageType::SirenStateResponse),
                
                StateChange::LockStateChange(state) => 
                    handle_state_change!(self, state, LockStateResponse, MessageType::LockStateResponse),
                
                StateChange::AlarmControlPanelStateChange(state) => 
                    handle_state_change!(self, state, AlarmControlPanelStateResponse, MessageType::AlarmControlPanelStateResponse),
                
                StateChange::TextStateChange(state) => 
                    handle_state_change!(self, state, TextStateResponse, MessageType::TextStateResponse),
                
                StateChange::DateStateChange(state) => 
                    handle_state_change!(self, state, DateStateResponse, MessageType::DateStateResponse),
                
                StateChange::TimeStateChange(state) => 
                    handle_state_change!(self, state, TimeStateResponse, MessageType::TimeStateResponse),
                
                StateChange::EventStateChange(state) => 
                    handle_state_change!(self, state, EventResponse, MessageType::EventResponse),
                
                StateChange::ValveStateChange(state) => 
                    handle_state_change!(self, state, ValveStateResponse, MessageType::ValveStateResponse),
                
                StateChange::DateTimeStateChange(state) => 
                    handle_state_change!(self, state, DateTimeStateResponse, MessageType::DateTimeStateResponse),
                
                StateChange::ClimateStateChange(state) => 
                    handle_state_change!(self, state, ClimateStateResponse, MessageType::ClimateStateResponse),
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
                log::info!("Handling HelloRequest, responding with name: {}", self.device_config.name);
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
                log::info!("Handling DeviceInfoRequest, responding with name: {}, mac: {}", self.device_config.name, self.device_config.mac_address);
                log::info!("DeviceConfig: {:?}", self.device_config);
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
                log::info!("Handling ListEntitiesRequest, responding with {} entities", self.entity_configs.len());
                //log::info!("EntityConfigs: {:?}", self.entity_configs);
                let _ = ListEntitiesRequest::decode(&data).map_err(|e| anyhow!(e))?;
                for entity in self.entity_configs.iter() {
                    match entity {
                        EntityConfig::BinarySensor(config) => 
                            handle_list_entity!(self, config, ListEntitiesBinarySensorResponse, MessageType::ListEntitiesBinarySensorResponse),
                        
                        EntityConfig::Switch(config) => 
                            handle_list_entity!(self, config, ListEntitiesSwitchResponse, MessageType::ListEntitiesSwitchResponse),
                        
                        EntityConfig::Sensor(config) => 
                            handle_list_entity!(self, config, ListEntitiesSensorResponse, MessageType::ListEntitiesSensorResponse),
                        
                        EntityConfig::TextSensor(config) => 
                            handle_list_entity!(self, config, ListEntitiesTextSensorResponse, MessageType::ListEntitiesTextSensorResponse),
                        
                        EntityConfig::Cover(config) => 
                            handle_list_entity!(self, config, ListEntitiesCoverResponse, MessageType::ListEntitiesCoverResponse),
                        
                        EntityConfig::Fan(config) => 
                            handle_list_entity!(self, config, ListEntitiesFanResponse, MessageType::ListEntitiesFanResponse),
                        
                        EntityConfig::Light(config) => 
                            handle_list_entity!(self, config, ListEntitiesLightResponse, MessageType::ListEntitiesLightResponse),
                        
                        EntityConfig::Climate(config) => 
                            handle_list_entity!(self, config, ListEntitiesClimateResponse, MessageType::ListEntitiesClimateResponse),
                        
                        EntityConfig::Number(config) => 
                            handle_list_entity!(self, config, ListEntitiesNumberResponse, MessageType::ListEntitiesNumberResponse),
                        
                        EntityConfig::Select(config) => 
                            handle_list_entity!(self, config, ListEntitiesSelectResponse, MessageType::ListEntitiesSelectResponse),
                        
                        EntityConfig::Siren(config) => 
                            handle_list_entity!(self, config, ListEntitiesSirenResponse, MessageType::ListEntitiesSirenResponse),
                        
                        EntityConfig::Lock(config) => 
                            handle_list_entity!(self, config, ListEntitiesLockResponse, MessageType::ListEntitiesLockResponse),
                        
                        EntityConfig::Button(config) => 
                            handle_list_entity!(self, config, ListEntitiesButtonResponse, MessageType::ListEntitiesButtonResponse),
                        
                        EntityConfig::AlarmControlPanel(config) => 
                            handle_list_entity!(self, config, ListEntitiesAlarmControlPanelResponse, MessageType::ListEntitiesAlarmControlPanelResponse),
                        
                        EntityConfig::Text(config) => 
                            handle_list_entity!(self, config, ListEntitiesTextResponse, MessageType::ListEntitiesTextResponse),
                        
                        EntityConfig::Date(config) => 
                            handle_list_entity!(self, config, ListEntitiesDateResponse, MessageType::ListEntitiesDateResponse),
                        
                        EntityConfig::Time(config) => 
                            handle_list_entity!(self, config, ListEntitiesTimeResponse, MessageType::ListEntitiesTimeResponse),
                        
                        EntityConfig::Event(config) => 
                            handle_list_entity!(self, config, ListEntitiesEventResponse, MessageType::ListEntitiesEventResponse),
                        
                        EntityConfig::Valve(config) => 
                            handle_list_entity!(self, config, ListEntitiesValveResponse, MessageType::ListEntitiesValveResponse),
                        
                        EntityConfig::DateTime(config) => 
                            handle_list_entity!(self, config, ListEntitiesDateTimeResponse, MessageType::ListEntitiesDateTimeResponse),
                    }
                }
                self.send(MessageType::ListEntitiesDoneResponse, &ListEntitiesDoneResponse::default()).await?;
            }

            MessageType::SubscribeStatesRequest => {
                let _ = SubscribeStatesRequest::decode(&data).map_err(|e| anyhow!(e))?;
                let mut status = self.connection.status.lock().await;
                status.subscribed_to_states = true;
                
                self.client_event_channel.send(ClientEvent::SubscribedToStates).await?;
            }

            MessageType::SubscribeLogsRequest => {
                let _ = SubscribeLogsRequest::decode(&data).map_err(|e| anyhow!(e))?;
                let mut status = self.connection.status.lock().await;
                status.subscribed_to_logs = true;
                
                self.client_event_channel.send(ClientEvent::SubscribedToLogs).await?;
            }

            MessageType::SwitchCommandRequest =>
                handle_command_request!(self, &data, SwitchCommand, SwitchCommandRequest),

            MessageType::CoverCommandRequest =>
                handle_command_request!(self, &data, CoverCommand, CoverCommandRequest),

            MessageType::FanCommandRequest =>
                handle_command_request!(self, &data, FanCommand, FanCommandRequest),

            MessageType::LightCommandRequest =>
                handle_command_request!(self, &data, LightCommand, LightCommandRequest),

            MessageType::ClimateCommandRequest =>
                handle_command_request!(self, &data, ClimateCommand, ClimateCommandRequest),

            MessageType::NumberCommandRequest =>
                handle_command_request!(self, &data, NumberCommand, NumberCommandRequest),

            MessageType::SelectCommandRequest =>
                handle_command_request!(self, &data, SelectCommand, SelectCommandRequest),

            MessageType::SirenCommandRequest =>
                handle_command_request!(self, &data, SirenCommand, SirenCommandRequest),

            MessageType::LockCommandRequest =>
                handle_command_request!(self, &data, LockCommand, LockCommandRequest),

            MessageType::ButtonCommandRequest =>
                handle_command_request!(self, &data, ButtonCommand, ButtonCommandRequest),

            MessageType::AlarmControlPanelCommandRequest =>
                handle_command_request!(self, &data, AlarmControlPanelCommand, AlarmControlPanelCommandRequest),

            MessageType::TextCommandRequest =>
                handle_command_request!(self, &data, TextCommand, TextCommandRequest),

            MessageType::DateCommandRequest =>
                handle_command_request!(self, &data, DateCommand, DateCommandRequest),

            MessageType::TimeCommandRequest =>
                handle_command_request!(self, &data, TimeCommand, TimeCommandRequest),

            MessageType::ValveCommandRequest =>
                handle_command_request!(self, &data, ValveCommand, ValveCommandRequest),

            MessageType::DateTimeCommandRequest =>
                handle_command_request!(self, &data, DateTimeCommand, DateTimeCommandRequest),

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

