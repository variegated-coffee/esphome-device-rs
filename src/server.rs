// Everything below `ConnectionStatus` is `#[cfg(feature = "std")]`, so these imports must be
// too. Without the gate they are seven `unused_imports` warnings in every `no_std` build --
// which is exactly what the embassy firmware consuming this crate saw, and the reason its
// build script documented eight warnings as unavoidable.
#[cfg(feature = "std")]
use crate::error::{EspHomeError, Result};
#[cfg(feature = "std")]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use femtopb::Message;
#[cfg(feature = "std")]
use crate::{ClientEvent, DeviceConfig, EntityConfig, StateChange};
#[cfg(all(feature = "std", feature = "_commandable"))]
use crate::Command;
#[cfg(feature = "std")]
use crate::metadata::MessageType;

// The protocol messages that are not tied to an entity type.
#[cfg(feature = "std")]
use crate::api::{
    ConnectRequest, ConnectResponse, DeviceInfoRequest, DeviceInfoResponse, DisconnectRequest,
    DisconnectResponse, HelloRequest, HelloResponse, ListEntitiesDoneResponse, ListEntitiesRequest,
    PingRequest, PingResponse, SubscribeLogsRequest, SubscribeStatesRequest,
};

// Per-entity-type protocol messages. `api.rs` is generated and deliberately left ungated, so
// these types always exist; the gates here are about not importing what this build's match
// arms no longer mention.
#[cfg(all(feature = "std", feature = "alarm_control_panel"))]
use crate::api::{
    AlarmControlPanelCommandRequest, AlarmControlPanelStateResponse,
    ListEntitiesAlarmControlPanelResponse,
};
#[cfg(all(feature = "std", feature = "binary_sensor"))]
use crate::api::{BinarySensorStateResponse, ListEntitiesBinarySensorResponse};
#[cfg(all(feature = "std", feature = "button"))]
use crate::api::{ButtonCommandRequest, ListEntitiesButtonResponse};
#[cfg(all(feature = "std", feature = "climate"))]
use crate::api::{ClimateCommandRequest, ClimateStateResponse, ListEntitiesClimateResponse};
#[cfg(all(feature = "std", feature = "cover"))]
use crate::api::{CoverCommandRequest, CoverStateResponse, ListEntitiesCoverResponse};
#[cfg(all(feature = "std", feature = "date"))]
use crate::api::{DateCommandRequest, DateStateResponse, ListEntitiesDateResponse};
#[cfg(all(feature = "std", feature = "datetime"))]
use crate::api::{DateTimeCommandRequest, DateTimeStateResponse, ListEntitiesDateTimeResponse};
#[cfg(all(feature = "std", feature = "event"))]
use crate::api::{EventResponse, ListEntitiesEventResponse};
#[cfg(all(feature = "std", feature = "fan"))]
use crate::api::{FanCommandRequest, FanStateResponse, ListEntitiesFanResponse};
#[cfg(all(feature = "std", feature = "light"))]
use crate::api::{LightCommandRequest, LightStateResponse, ListEntitiesLightResponse};
#[cfg(all(feature = "std", feature = "lock"))]
use crate::api::{ListEntitiesLockResponse, LockCommandRequest, LockStateResponse};
#[cfg(all(feature = "std", feature = "number"))]
use crate::api::{ListEntitiesNumberResponse, NumberCommandRequest, NumberStateResponse};
#[cfg(all(feature = "std", feature = "select"))]
use crate::api::{ListEntitiesSelectResponse, SelectCommandRequest, SelectStateResponse};
#[cfg(all(feature = "std", feature = "sensor"))]
use crate::api::{ListEntitiesSensorResponse, SensorStateResponse};
#[cfg(all(feature = "std", feature = "siren"))]
use crate::api::{ListEntitiesSirenResponse, SirenCommandRequest, SirenStateResponse};
#[cfg(all(feature = "std", feature = "switch"))]
use crate::api::{ListEntitiesSwitchResponse, SwitchCommandRequest, SwitchStateResponse};
#[cfg(all(feature = "std", feature = "text"))]
use crate::api::{ListEntitiesTextResponse, TextCommandRequest, TextStateResponse};
#[cfg(all(feature = "std", feature = "text_sensor"))]
use crate::api::{ListEntitiesTextSensorResponse, TextSensorStateResponse};
#[cfg(all(feature = "std", feature = "time"))]
use crate::api::{ListEntitiesTimeResponse, TimeCommandRequest, TimeStateResponse};
#[cfg(all(feature = "std", feature = "valve"))]
use crate::api::{ListEntitiesValveResponse, ValveCommandRequest, ValveStateResponse};

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

// Each macro is used only by the arms of one of the three matches below, so each carries the
// capability marker as well as `std`. See the internal markers in `Cargo.toml`.
#[cfg(all(feature = "std", feature = "_commandable"))]
macro_rules! handle_command_request {
    ($self:expr, $data:expr, $command_variant:ident, $request_type:ty) => {
        $self.client_event_channel.send(
            ClientEvent::CommandReceived(
                Command::$command_variant(
                    <$request_type>::decode($data)
                        .map_err(|_| EspHomeError::DecodeError)?
                        .into()
                )
            )
        ).await?
    };
}

#[cfg(all(feature = "std", feature = "_reportable"))]
macro_rules! handle_state_change {
    ($self:expr, $state:expr, $response_type:ty, $msg_type:expr) => {
        $self.send::<$response_type>($msg_type, &$state.into()).await?
    };
}

#[cfg(all(feature = "std", feature = "_listable"))]
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

    async fn read_frame(&self) -> Result<(MessageType, Vec<u8>)> {
        self.connection.read_frame().await
    }

    async fn send<'m, M: Message<'m>>(&self, msg_type: MessageType, message: &'m M) -> Result<()> {
        log::debug!("Sending message of type {:?}", msg_type);
        self.connection.send(msg_type, message).await
    }

    pub async fn run(&mut self) -> Result<()> {
        let _ = join!(self.run_channel_loop(), self.run_socket_loop());

        Ok(())
    }

    pub async fn run_channel_loop(&self) -> Result<()> {
        loop {
            let state_change = self.state_change_channel.recv().await?;

            let _status = self.connection.status.lock().await;
/*            if !status.subscribed_to_states {
                log::warn!("Not subscribed to states, skipping state change");
                continue;
            }*/

            //log::info!("State change received");

            // One arm per enabled entity type. The `_Uninhabited` variant needs none: it
            // carries `Infallible`, and `min_exhaustive_patterns` lets the compiler see that
            // it cannot be constructed. A build with no entity types leaves this match with
            // zero arms over an uninhabited enum, which is also fine.
            match state_change {
                #[cfg(feature = "binary_sensor")]
                StateChange::BinarySensorChange(state) =>
                    handle_state_change!(self, state, BinarySensorStateResponse, MessageType::BinarySensorStateResponse),

                #[cfg(feature = "cover")]
                StateChange::CoverChange(state) =>
                    handle_state_change!(self, state, CoverStateResponse, MessageType::CoverStateResponse),

                #[cfg(feature = "switch")]
                StateChange::SwitchStateChange(state) =>
                    handle_state_change!(self, state, SwitchStateResponse, MessageType::SwitchStateResponse),

                #[cfg(feature = "fan")]
                StateChange::FanStateChange(state) =>
                    handle_state_change!(self, state, FanStateResponse, MessageType::FanStateResponse),

                #[cfg(feature = "light")]
                StateChange::LightStateChange(state) =>
                    handle_state_change!(self, state, LightStateResponse, MessageType::LightStateResponse),

                #[cfg(feature = "sensor")]
                StateChange::SensorStateChange(state) =>
                    handle_state_change!(self, state, SensorStateResponse, MessageType::SensorStateResponse),

                #[cfg(feature = "text_sensor")]
                StateChange::TextSensorStateChange(state) =>
                    handle_state_change!(self, state, TextSensorStateResponse, MessageType::TextSensorStateResponse),

                #[cfg(feature = "number")]
                StateChange::NumberStateChange(state) =>
                    handle_state_change!(self, state, NumberStateResponse, MessageType::NumberStateResponse),

                #[cfg(feature = "select")]
                StateChange::SelectStateChange(state) =>
                    handle_state_change!(self, state, SelectStateResponse, MessageType::SelectStateResponse),

                #[cfg(feature = "siren")]
                StateChange::SirenStateChange(state) =>
                    handle_state_change!(self, state, SirenStateResponse, MessageType::SirenStateResponse),

                #[cfg(feature = "lock")]
                StateChange::LockStateChange(state) =>
                    handle_state_change!(self, state, LockStateResponse, MessageType::LockStateResponse),

                #[cfg(feature = "alarm_control_panel")]
                StateChange::AlarmControlPanelStateChange(state) =>
                    handle_state_change!(self, state, AlarmControlPanelStateResponse, MessageType::AlarmControlPanelStateResponse),

                #[cfg(feature = "text")]
                StateChange::TextStateChange(state) =>
                    handle_state_change!(self, state, TextStateResponse, MessageType::TextStateResponse),

                #[cfg(feature = "date")]
                StateChange::DateStateChange(state) =>
                    handle_state_change!(self, state, DateStateResponse, MessageType::DateStateResponse),

                #[cfg(feature = "time")]
                StateChange::TimeStateChange(state) =>
                    handle_state_change!(self, state, TimeStateResponse, MessageType::TimeStateResponse),

                #[cfg(feature = "event")]
                StateChange::EventStateChange(state) =>
                    handle_state_change!(self, state, EventResponse, MessageType::EventResponse),

                #[cfg(feature = "valve")]
                StateChange::ValveStateChange(state) =>
                    handle_state_change!(self, state, ValveStateResponse, MessageType::ValveStateResponse),

                #[cfg(feature = "datetime")]
                StateChange::DateTimeStateChange(state) =>
                    handle_state_change!(self, state, DateTimeStateResponse, MessageType::DateTimeStateResponse),

                #[cfg(feature = "climate")]
                StateChange::ClimateStateChange(state) =>
                    handle_state_change!(self, state, ClimateStateResponse, MessageType::ClimateStateResponse),
            }
        }
    }

    pub async fn run_socket_loop(&self) -> Result<()> {
        loop {
            let (message_type, frame_data) = self.read_frame().await?;
            self.handle_message(message_type, frame_data).await?;
        }
    }

    async fn validate_status(&self, message_type: MessageType) -> Result<()> {
        let status = self.connection.status.lock().await;
        if message_type.needs_authentication() && !status.authenticated {
            return Err(EspHomeError::NotAuthenticated);
        }
        if message_type.needs_setup_connection() && !status.setup_complete {
            return Err(EspHomeError::ConnectionNotSetup);
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
                log::info!("Handling HelloRequest, responding with name: {}", self.device_config.name);
                let _ = HelloRequest::decode(&data).map_err(|_| EspHomeError::DecodeError)?;
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
                let request = ConnectRequest::decode(&data).map_err(|_| EspHomeError::DecodeError)?;

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
                let _ = DisconnectRequest::decode(&data).map_err(|_| EspHomeError::DecodeError)?;
                self.send(MessageType::DisconnectResponse, &DisconnectResponse::default()).await?;
                
                log::info!("Client requested disconnect");
                // Close the connection
            }

            MessageType::DeviceInfoRequest => {
                log::info!("Handling DeviceInfoRequest, responding with name: {}, mac: {}", self.device_config.name, self.device_config.mac_address);
                log::info!("DeviceConfig: {:?}", self.device_config);
                let _ = DeviceInfoRequest::decode(&data).map_err(|_| EspHomeError::DecodeError)?;
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
                let _ = PingRequest::decode(&data).map_err(|_| EspHomeError::DecodeError)?;
                self.send(MessageType::PingResponse, &PingResponse::default()).await?;
            }

            MessageType::ListEntitiesRequest => {
                log::info!("Handling ListEntitiesRequest, responding with {} entities", self.entity_configs.len());
                //log::info!("EntityConfigs: {:?}", self.entity_configs);
                let _ = ListEntitiesRequest::decode(&data).map_err(|_| EspHomeError::DecodeError)?;
                for entity in self.entity_configs.iter() {
                    match entity {
                        // Cannot run: `_Uninhabited` carries `Infallible`. The arm is here
                        // because this match is over a `&EntityConfig`, and exhaustiveness
                        // checking does not see through the reference the way it does for the
                        // `StateChange` match by value -- reading an uninhabited place behind
                        // a reference is what `match *never {}` states is impossible.
                        EntityConfig::_Uninhabited(_, never) => match *never {},

                        #[cfg(feature = "binary_sensor")]
                        EntityConfig::BinarySensor(config) =>
                            handle_list_entity!(self, config, ListEntitiesBinarySensorResponse, MessageType::ListEntitiesBinarySensorResponse),

                        #[cfg(feature = "switch")]
                        EntityConfig::Switch(config) =>
                            handle_list_entity!(self, config, ListEntitiesSwitchResponse, MessageType::ListEntitiesSwitchResponse),

                        #[cfg(feature = "sensor")]
                        EntityConfig::Sensor(config) =>
                            handle_list_entity!(self, config, ListEntitiesSensorResponse, MessageType::ListEntitiesSensorResponse),

                        #[cfg(feature = "text_sensor")]
                        EntityConfig::TextSensor(config) =>
                            handle_list_entity!(self, config, ListEntitiesTextSensorResponse, MessageType::ListEntitiesTextSensorResponse),

                        #[cfg(feature = "cover")]
                        EntityConfig::Cover(config) =>
                            handle_list_entity!(self, config, ListEntitiesCoverResponse, MessageType::ListEntitiesCoverResponse),

                        #[cfg(feature = "fan")]
                        EntityConfig::Fan(config) =>
                            handle_list_entity!(self, config, ListEntitiesFanResponse, MessageType::ListEntitiesFanResponse),

                        #[cfg(feature = "light")]
                        EntityConfig::Light(config) =>
                            handle_list_entity!(self, config, ListEntitiesLightResponse, MessageType::ListEntitiesLightResponse),

                        #[cfg(feature = "climate")]
                        EntityConfig::Climate(config) =>
                            handle_list_entity!(self, config, ListEntitiesClimateResponse, MessageType::ListEntitiesClimateResponse),

                        #[cfg(feature = "number")]
                        EntityConfig::Number(config) =>
                            handle_list_entity!(self, config, ListEntitiesNumberResponse, MessageType::ListEntitiesNumberResponse),

                        #[cfg(feature = "select")]
                        EntityConfig::Select(config) =>
                            handle_list_entity!(self, config, ListEntitiesSelectResponse, MessageType::ListEntitiesSelectResponse),

                        #[cfg(feature = "siren")]
                        EntityConfig::Siren(config) =>
                            handle_list_entity!(self, config, ListEntitiesSirenResponse, MessageType::ListEntitiesSirenResponse),

                        #[cfg(feature = "lock")]
                        EntityConfig::Lock(config) =>
                            handle_list_entity!(self, config, ListEntitiesLockResponse, MessageType::ListEntitiesLockResponse),

                        #[cfg(feature = "button")]
                        EntityConfig::Button(config) =>
                            handle_list_entity!(self, config, ListEntitiesButtonResponse, MessageType::ListEntitiesButtonResponse),

                        #[cfg(feature = "alarm_control_panel")]
                        EntityConfig::AlarmControlPanel(config) =>
                            handle_list_entity!(self, config, ListEntitiesAlarmControlPanelResponse, MessageType::ListEntitiesAlarmControlPanelResponse),

                        #[cfg(feature = "text")]
                        EntityConfig::Text(config) =>
                            handle_list_entity!(self, config, ListEntitiesTextResponse, MessageType::ListEntitiesTextResponse),

                        #[cfg(feature = "date")]
                        EntityConfig::Date(config) =>
                            handle_list_entity!(self, config, ListEntitiesDateResponse, MessageType::ListEntitiesDateResponse),

                        #[cfg(feature = "time")]
                        EntityConfig::Time(config) =>
                            handle_list_entity!(self, config, ListEntitiesTimeResponse, MessageType::ListEntitiesTimeResponse),

                        #[cfg(feature = "event")]
                        EntityConfig::Event(config) =>
                            handle_list_entity!(self, config, ListEntitiesEventResponse, MessageType::ListEntitiesEventResponse),

                        #[cfg(feature = "valve")]
                        EntityConfig::Valve(config) =>
                            handle_list_entity!(self, config, ListEntitiesValveResponse, MessageType::ListEntitiesValveResponse),

                        #[cfg(feature = "datetime")]
                        EntityConfig::DateTime(config) =>
                            handle_list_entity!(self, config, ListEntitiesDateTimeResponse, MessageType::ListEntitiesDateTimeResponse),
                    }
                }
                self.send(MessageType::ListEntitiesDoneResponse, &ListEntitiesDoneResponse::default()).await?;
            }

            MessageType::SubscribeStatesRequest => {
                let _ = SubscribeStatesRequest::decode(&data).map_err(|_| EspHomeError::DecodeError)?;
                let mut status = self.connection.status.lock().await;
                status.subscribed_to_states = true;
                
                self.client_event_channel.send(ClientEvent::SubscribedToStates).await?;
            }

            MessageType::SubscribeLogsRequest => {
                let _ = SubscribeLogsRequest::decode(&data).map_err(|_| EspHomeError::DecodeError)?;
                let mut status = self.connection.status.lock().await;
                status.subscribed_to_logs = true;
                
                self.client_event_channel.send(ClientEvent::SubscribedToLogs).await?;
            }

            // Unlike the two matches above, this one already has a catch-all, so removing an
            // arm is not a compile break -- a command for a type this build does not carry
            // falls through to `UnsupportedMessageType` below. A client cannot send one
            // anyway: it only learns of entities this device listed.
            #[cfg(feature = "switch")]
            MessageType::SwitchCommandRequest =>
                handle_command_request!(self, &data, SwitchCommand, SwitchCommandRequest),

            #[cfg(feature = "cover")]
            MessageType::CoverCommandRequest =>
                handle_command_request!(self, &data, CoverCommand, CoverCommandRequest),

            #[cfg(feature = "fan")]
            MessageType::FanCommandRequest =>
                handle_command_request!(self, &data, FanCommand, FanCommandRequest),

            #[cfg(feature = "light")]
            MessageType::LightCommandRequest =>
                handle_command_request!(self, &data, LightCommand, LightCommandRequest),

            #[cfg(feature = "climate")]
            MessageType::ClimateCommandRequest =>
                handle_command_request!(self, &data, ClimateCommand, ClimateCommandRequest),

            #[cfg(feature = "number")]
            MessageType::NumberCommandRequest =>
                handle_command_request!(self, &data, NumberCommand, NumberCommandRequest),

            #[cfg(feature = "select")]
            MessageType::SelectCommandRequest =>
                handle_command_request!(self, &data, SelectCommand, SelectCommandRequest),

            #[cfg(feature = "siren")]
            MessageType::SirenCommandRequest =>
                handle_command_request!(self, &data, SirenCommand, SirenCommandRequest),

            #[cfg(feature = "lock")]
            MessageType::LockCommandRequest =>
                handle_command_request!(self, &data, LockCommand, LockCommandRequest),

            #[cfg(feature = "button")]
            MessageType::ButtonCommandRequest =>
                handle_command_request!(self, &data, ButtonCommand, ButtonCommandRequest),

            #[cfg(feature = "alarm_control_panel")]
            MessageType::AlarmControlPanelCommandRequest =>
                handle_command_request!(self, &data, AlarmControlPanelCommand, AlarmControlPanelCommandRequest),

            #[cfg(feature = "text")]
            MessageType::TextCommandRequest =>
                handle_command_request!(self, &data, TextCommand, TextCommandRequest),

            #[cfg(feature = "date")]
            MessageType::DateCommandRequest =>
                handle_command_request!(self, &data, DateCommand, DateCommandRequest),

            #[cfg(feature = "time")]
            MessageType::TimeCommandRequest =>
                handle_command_request!(self, &data, TimeCommand, TimeCommandRequest),

            #[cfg(feature = "valve")]
            MessageType::ValveCommandRequest =>
                handle_command_request!(self, &data, ValveCommand, ValveCommandRequest),

            #[cfg(feature = "datetime")]
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
                return Err(EspHomeError::UnsupportedMessageType);
            }
        }

        Ok(())
    }
}

