use crate::error::{EspHomeError, Result};
use embassy_futures::select::{select, Either};
use alloc::vec::Vec;
use femtopb::{Message};
use embassy_net::tcp::{TcpReader, TcpWriter};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::channel::{Receiver, Sender};
use crate::metadata::MessageType;
use crate::server::ConnectionStatus;
use crate::embassy_net::frame_reader::EspHomeFrameReader;
use crate::embassy_net::message_sender::MessageSender;
use crate::{ClientEvent, DeviceConfig, EntityConfig, StateChange};
#[cfg(feature = "_commandable")]
use crate::Command;
// The protocol messages that are not tied to an entity type.
use crate::api::{
    ConnectRequest, ConnectResponse, DeviceInfoRequest, DeviceInfoResponse, DisconnectRequest,
    DisconnectResponse, HelloRequest, HelloResponse, ListEntitiesDoneResponse, ListEntitiesRequest,
    PingRequest, PingResponse, SubscribeLogsRequest, SubscribeStatesRequest,
};

// Per-entity-type protocol messages. `api.rs` is generated and deliberately left ungated, so
// these types always exist; the gates here are about not importing what this build's match
// arms no longer mention.
#[cfg(feature = "alarm_control_panel")]
use crate::api::{
    AlarmControlPanelCommandRequest, AlarmControlPanelStateResponse,
    ListEntitiesAlarmControlPanelResponse,
};
#[cfg(feature = "binary_sensor")]
use crate::api::{BinarySensorStateResponse, ListEntitiesBinarySensorResponse};
#[cfg(feature = "button")]
use crate::api::{ButtonCommandRequest, ListEntitiesButtonResponse};
#[cfg(feature = "climate")]
use crate::api::{ClimateCommandRequest, ClimateStateResponse, ListEntitiesClimateResponse};
#[cfg(feature = "cover")]
use crate::api::{CoverCommandRequest, CoverStateResponse, ListEntitiesCoverResponse};
#[cfg(feature = "date")]
use crate::api::{DateCommandRequest, DateStateResponse, ListEntitiesDateResponse};
#[cfg(feature = "datetime")]
use crate::api::{DateTimeCommandRequest, DateTimeStateResponse, ListEntitiesDateTimeResponse};
#[cfg(feature = "event")]
use crate::api::{EventResponse, ListEntitiesEventResponse};
#[cfg(feature = "fan")]
use crate::api::{FanCommandRequest, FanStateResponse, ListEntitiesFanResponse};
#[cfg(feature = "light")]
use crate::api::{LightCommandRequest, LightStateResponse, ListEntitiesLightResponse};
#[cfg(feature = "lock")]
use crate::api::{ListEntitiesLockResponse, LockCommandRequest, LockStateResponse};
#[cfg(feature = "number")]
use crate::api::{ListEntitiesNumberResponse, NumberCommandRequest, NumberStateResponse};
#[cfg(feature = "select")]
use crate::api::{ListEntitiesSelectResponse, SelectCommandRequest, SelectStateResponse};
#[cfg(feature = "sensor")]
use crate::api::{ListEntitiesSensorResponse, SensorStateResponse};
#[cfg(feature = "siren")]
use crate::api::{ListEntitiesSirenResponse, SirenCommandRequest, SirenStateResponse};
#[cfg(feature = "switch")]
use crate::api::{ListEntitiesSwitchResponse, SwitchCommandRequest, SwitchStateResponse};
#[cfg(feature = "text")]
use crate::api::{ListEntitiesTextResponse, TextCommandRequest, TextStateResponse};
#[cfg(feature = "text_sensor")]
use crate::api::{ListEntitiesTextSensorResponse, TextSensorStateResponse};
#[cfg(feature = "time")]
use crate::api::{ListEntitiesTimeResponse, TimeCommandRequest, TimeStateResponse};
#[cfg(feature = "valve")]
use crate::api::{ListEntitiesValveResponse, ValveCommandRequest, ValveStateResponse};

/// ESPHome connection handler for embassy-net
pub struct EspHomeConnection<'a> {
    reader: Mutex<CriticalSectionRawMutex, Option<EspHomeFrameReader<'a>>>,
    writer: Mutex<CriticalSectionRawMutex, Option<MessageSender<'a>>>,
    pub status: Mutex<CriticalSectionRawMutex, ConnectionStatus>,
}

impl<'a> EspHomeConnection<'a> {
    pub fn new(reader: &'a mut TcpReader<'a>, writer: &'a mut TcpWriter<'a>) -> Self {
        let reader_wrapper = EspHomeFrameReader::new(reader, 4096);
        let writer_wrapper = MessageSender::new(writer);

        Self {
            reader: Mutex::new(Some(reader_wrapper)),
            writer: Mutex::new(Some(writer_wrapper)),
            status: Mutex::new(ConnectionStatus::default()),
        }
    }

    pub async fn send<'m, M: Message<'m>>(&self, msg_type: MessageType, message: &'m M) -> Result<()> {
        let mut writer_guard = self.writer.lock().await;
        if let Some(writer) = writer_guard.as_mut() {
            writer.send(msg_type, message).await
        } else {
            Err(EspHomeError::WriterNotAvailable)
        }
    }

    pub async fn read_frame(&self) -> Result<(MessageType, Vec<u8>)> {
        let mut reader_guard = self.reader.lock().await;
        if let Some(reader) = reader_guard.as_mut() {
            reader.read_frame().await
        } else {
            Err(EspHomeError::ReaderNotAvailable)
        }
    }
}

pub struct EspHomeServer<'a, 's, 'c, const STATE_CAPACITY: usize, const EVENT_CAPACITY: usize> {
    connection: &'c EspHomeConnection<'a>,
    device_config: &'a DeviceConfig<'a>,
    state_change_channel: &'c Receiver<'c, CriticalSectionRawMutex, StateChange<'s>, STATE_CAPACITY>,
    client_event_channel: &'c Sender<'c, CriticalSectionRawMutex, ClientEvent, EVENT_CAPACITY>,
    entity_configs: &'a[EntityConfig<'a>],
}

// Each macro is used only by the arms of one of the three matches below, so each is gated on
// whether any enabled entity type has that capability. See the internal markers in
// `Cargo.toml`.
#[cfg(feature = "_commandable")]
macro_rules! handle_command_request_embassy {
    ($self:expr, $data:expr, $command_variant:ident, $request_type:ty) => {
        $self.client_event_channel.send(
            ClientEvent::CommandReceived(
                Command::$command_variant(
                    <$request_type>::decode($data)
                        .map_err(|_| EspHomeError::DecodeError)?
                        .into()
                )
            )
        ).await
    };
}

#[cfg(feature = "_reportable")]
macro_rules! handle_state_change_embassy {
    ($self:expr, $state:expr, $response_type:ty, $msg_type:expr) => {
        $self.send::<$response_type>($msg_type, &$state.into()).await?
    };
}

#[cfg(feature = "_listable")]
macro_rules! handle_list_entity_embassy {
    ($self:expr, $config:expr, $response_type:ty, $msg_type:expr) => {
        $self.send::<$response_type>($msg_type, &$config.into()).await?
    };
}

impl<'a, 's, 'c, const STATE_CAPACITY: usize, const EVENT_CAPACITY: usize> EspHomeServer<'a, 's, 'c, STATE_CAPACITY, EVENT_CAPACITY> {
    pub fn new(
        connection: &'c EspHomeConnection<'a>,
        device_config: &'a DeviceConfig<'a>,
        entity_configs: &'a[EntityConfig<'a>],
        state_change_channel: &'c Receiver<'c, CriticalSectionRawMutex, StateChange<'s>, STATE_CAPACITY>,
        client_event_channel: &'c Sender<'c, CriticalSectionRawMutex, ClientEvent, EVENT_CAPACITY>,
    ) -> Self {
        // The std backend marks a password-less config authenticated here via
        // `lock_blocking()`, which embassy_sync's Mutex does not offer. Rather
        // than reach for `try_lock` in a constructor, `validate_status` treats
        // "no password configured" as authenticated -- same effect, and it
        // cannot fail. Without either, a `password: None` device would reject
        // every request that needs authentication.

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

    /// Drive the connection until either loop fails.
    ///
    /// The socket loop reads and dispatches inbound frames; the channel loop
    /// pushes outbound state changes. Both run until one returns an error
    /// (normally `ConnectionClosed`), which is what the caller uses to tear the
    /// connection down. `select` rather than `join` because the first failure
    /// should end the connection immediately.
    pub async fn run(&self) -> Result<()> {
        match select(self.run_socket_loop(), self.run_channel_loop()).await {
            Either::First(result) => result,
            Either::Second(result) => result,
        }
    }

    async fn validate_status(&self, message_type: MessageType) -> Result<()> {
        let status = self.connection.status.lock().await;
        // A device with no password is authenticated from the start; see `new`.
        let authenticated = status.authenticated || self.device_config.password.is_none();
        if message_type.needs_authentication() && !authenticated {
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

                if !response.invalid_password {
                    let mut status = self.connection.status.lock().await;
                    status.authenticated = true;
                }
            }

            MessageType::DisconnectRequest => {
                let _ = DisconnectRequest::decode(&data).map_err(|_| EspHomeError::DecodeError)?;
                self.send(MessageType::DisconnectResponse, &DisconnectResponse::default()).await?;

                log::info!("Client requested disconnect");
            }

            MessageType::DeviceInfoRequest => {
                log::info!("Handling DeviceInfoRequest, responding with name: {}, mac: {}", self.device_config.name, self.device_config.mac_address);
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
                let _ = ListEntitiesRequest::decode(&data).map_err(|_| EspHomeError::DecodeError)?;
                for entity in self.entity_configs.iter() {
                    match entity {
                        // See the twin in `crate::server` for why this arm exists and why it
                        // cannot run.
                        EntityConfig::_Uninhabited(_, never) => match *never {},

                        #[cfg(feature = "binary_sensor")]
                        EntityConfig::BinarySensor(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesBinarySensorResponse, MessageType::ListEntitiesBinarySensorResponse),

                        #[cfg(feature = "switch")]
                        EntityConfig::Switch(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesSwitchResponse, MessageType::ListEntitiesSwitchResponse),

                        #[cfg(feature = "sensor")]
                        EntityConfig::Sensor(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesSensorResponse, MessageType::ListEntitiesSensorResponse),

                        #[cfg(feature = "text_sensor")]
                        EntityConfig::TextSensor(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesTextSensorResponse, MessageType::ListEntitiesTextSensorResponse),

                        #[cfg(feature = "cover")]
                        EntityConfig::Cover(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesCoverResponse, MessageType::ListEntitiesCoverResponse),

                        #[cfg(feature = "fan")]
                        EntityConfig::Fan(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesFanResponse, MessageType::ListEntitiesFanResponse),

                        #[cfg(feature = "light")]
                        EntityConfig::Light(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesLightResponse, MessageType::ListEntitiesLightResponse),

                        #[cfg(feature = "climate")]
                        EntityConfig::Climate(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesClimateResponse, MessageType::ListEntitiesClimateResponse),

                        #[cfg(feature = "number")]
                        EntityConfig::Number(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesNumberResponse, MessageType::ListEntitiesNumberResponse),

                        #[cfg(feature = "select")]
                        EntityConfig::Select(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesSelectResponse, MessageType::ListEntitiesSelectResponse),

                        #[cfg(feature = "siren")]
                        EntityConfig::Siren(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesSirenResponse, MessageType::ListEntitiesSirenResponse),

                        #[cfg(feature = "lock")]
                        EntityConfig::Lock(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesLockResponse, MessageType::ListEntitiesLockResponse),

                        #[cfg(feature = "button")]
                        EntityConfig::Button(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesButtonResponse, MessageType::ListEntitiesButtonResponse),

                        #[cfg(feature = "alarm_control_panel")]
                        EntityConfig::AlarmControlPanel(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesAlarmControlPanelResponse, MessageType::ListEntitiesAlarmControlPanelResponse),

                        #[cfg(feature = "text")]
                        EntityConfig::Text(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesTextResponse, MessageType::ListEntitiesTextResponse),

                        #[cfg(feature = "date")]
                        EntityConfig::Date(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesDateResponse, MessageType::ListEntitiesDateResponse),

                        #[cfg(feature = "time")]
                        EntityConfig::Time(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesTimeResponse, MessageType::ListEntitiesTimeResponse),

                        #[cfg(feature = "event")]
                        EntityConfig::Event(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesEventResponse, MessageType::ListEntitiesEventResponse),

                        #[cfg(feature = "valve")]
                        EntityConfig::Valve(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesValveResponse, MessageType::ListEntitiesValveResponse),

                        #[cfg(feature = "datetime")]
                        EntityConfig::DateTime(config) =>
                            handle_list_entity_embassy!(self, config, ListEntitiesDateTimeResponse, MessageType::ListEntitiesDateTimeResponse),
                    }
                }
                self.send(MessageType::ListEntitiesDoneResponse, &ListEntitiesDoneResponse::default()).await?;
            }

            MessageType::SubscribeStatesRequest => {
                let _ = SubscribeStatesRequest::decode(&data).map_err(|_| EspHomeError::DecodeError)?;
                let mut status = self.connection.status.lock().await;
                status.subscribed_to_states = true;

                self.client_event_channel.send(ClientEvent::SubscribedToStates).await;
            }

            MessageType::SubscribeLogsRequest => {
                let _ = SubscribeLogsRequest::decode(&data).map_err(|_| EspHomeError::DecodeError)?;
                let mut status = self.connection.status.lock().await;
                status.subscribed_to_logs = true;

                self.client_event_channel.send(ClientEvent::SubscribedToLogs).await;
            }

            // This match already ends in a catch-all, so removing an arm is not a compile
            // break -- a command for a type this build does not carry is logged and dropped.
            // A client cannot send one anyway: it only learns of entities this device listed.
            #[cfg(feature = "switch")]
            MessageType::SwitchCommandRequest =>
                handle_command_request_embassy!(self, &data, SwitchCommand, SwitchCommandRequest),

            #[cfg(feature = "cover")]
            MessageType::CoverCommandRequest =>
                handle_command_request_embassy!(self, &data, CoverCommand, CoverCommandRequest),

            #[cfg(feature = "fan")]
            MessageType::FanCommandRequest =>
                handle_command_request_embassy!(self, &data, FanCommand, FanCommandRequest),

            #[cfg(feature = "light")]
            MessageType::LightCommandRequest =>
                handle_command_request_embassy!(self, &data, LightCommand, LightCommandRequest),

            #[cfg(feature = "climate")]
            MessageType::ClimateCommandRequest =>
                handle_command_request_embassy!(self, &data, ClimateCommand, ClimateCommandRequest),

            #[cfg(feature = "number")]
            MessageType::NumberCommandRequest =>
                handle_command_request_embassy!(self, &data, NumberCommand, NumberCommandRequest),

            #[cfg(feature = "select")]
            MessageType::SelectCommandRequest =>
                handle_command_request_embassy!(self, &data, SelectCommand, SelectCommandRequest),

            #[cfg(feature = "siren")]
            MessageType::SirenCommandRequest =>
                handle_command_request_embassy!(self, &data, SirenCommand, SirenCommandRequest),

            #[cfg(feature = "lock")]
            MessageType::LockCommandRequest =>
                handle_command_request_embassy!(self, &data, LockCommand, LockCommandRequest),

            #[cfg(feature = "button")]
            MessageType::ButtonCommandRequest =>
                handle_command_request_embassy!(self, &data, ButtonCommand, ButtonCommandRequest),

            #[cfg(feature = "alarm_control_panel")]
            MessageType::AlarmControlPanelCommandRequest =>
                handle_command_request_embassy!(self, &data, AlarmControlPanelCommand, AlarmControlPanelCommandRequest),

            #[cfg(feature = "text")]
            MessageType::TextCommandRequest =>
                handle_command_request_embassy!(self, &data, TextCommand, TextCommandRequest),

            #[cfg(feature = "date")]
            MessageType::DateCommandRequest =>
                handle_command_request_embassy!(self, &data, DateCommand, DateCommandRequest),

            #[cfg(feature = "time")]
            MessageType::TimeCommandRequest =>
                handle_command_request_embassy!(self, &data, TimeCommand, TimeCommandRequest),

            #[cfg(feature = "valve")]
            MessageType::ValveCommandRequest =>
                handle_command_request_embassy!(self, &data, ValveCommand, ValveCommandRequest),

            #[cfg(feature = "datetime")]
            MessageType::DateTimeCommandRequest =>
                handle_command_request_embassy!(self, &data, DateTimeCommand, DateTimeCommandRequest),

            _ => {
                log::warn!("Unsupported message type: {:?}", msg_type);
            }
        }

        Ok(())
    }

    pub async fn run_socket_loop(&self) -> Result<()> {
        loop {
            let (message_type, frame_data) = self.read_frame().await?;
            self.handle_message(message_type, frame_data).await?;
        }
    }

    pub async fn run_channel_loop(&self) -> Result<()> {
        loop {
            let state_change = self.state_change_channel.receive().await;

            // One arm per enabled entity type. `_Uninhabited` needs none here: this match is
            // by value, so `min_exhaustive_patterns` sees the `Infallible` and does not ask
            // for it -- unlike the `&EntityConfig` match above.
            match state_change {
                #[cfg(feature = "binary_sensor")]
                StateChange::BinarySensorChange(state) =>
                    handle_state_change_embassy!(self, state, BinarySensorStateResponse, MessageType::BinarySensorStateResponse),

                #[cfg(feature = "cover")]
                StateChange::CoverChange(state) =>
                    handle_state_change_embassy!(self, state, CoverStateResponse, MessageType::CoverStateResponse),

                #[cfg(feature = "switch")]
                StateChange::SwitchStateChange(state) =>
                    handle_state_change_embassy!(self, state, SwitchStateResponse, MessageType::SwitchStateResponse),

                #[cfg(feature = "fan")]
                StateChange::FanStateChange(state) =>
                    handle_state_change_embassy!(self, state, FanStateResponse, MessageType::FanStateResponse),

                #[cfg(feature = "light")]
                StateChange::LightStateChange(state) =>
                    handle_state_change_embassy!(self, state, LightStateResponse, MessageType::LightStateResponse),

                #[cfg(feature = "sensor")]
                StateChange::SensorStateChange(state) =>
                    handle_state_change_embassy!(self, state, SensorStateResponse, MessageType::SensorStateResponse),

                #[cfg(feature = "text_sensor")]
                StateChange::TextSensorStateChange(state) =>
                    handle_state_change_embassy!(self, state, TextSensorStateResponse, MessageType::TextSensorStateResponse),

                #[cfg(feature = "number")]
                StateChange::NumberStateChange(state) =>
                    handle_state_change_embassy!(self, state, NumberStateResponse, MessageType::NumberStateResponse),

                #[cfg(feature = "select")]
                StateChange::SelectStateChange(state) =>
                    handle_state_change_embassy!(self, state, SelectStateResponse, MessageType::SelectStateResponse),

                #[cfg(feature = "siren")]
                StateChange::SirenStateChange(state) =>
                    handle_state_change_embassy!(self, state, SirenStateResponse, MessageType::SirenStateResponse),

                #[cfg(feature = "lock")]
                StateChange::LockStateChange(state) =>
                    handle_state_change_embassy!(self, state, LockStateResponse, MessageType::LockStateResponse),

                #[cfg(feature = "alarm_control_panel")]
                StateChange::AlarmControlPanelStateChange(state) =>
                    handle_state_change_embassy!(self, state, AlarmControlPanelStateResponse, MessageType::AlarmControlPanelStateResponse),

                #[cfg(feature = "text")]
                StateChange::TextStateChange(state) =>
                    handle_state_change_embassy!(self, state, TextStateResponse, MessageType::TextStateResponse),

                #[cfg(feature = "date")]
                StateChange::DateStateChange(state) =>
                    handle_state_change_embassy!(self, state, DateStateResponse, MessageType::DateStateResponse),

                #[cfg(feature = "time")]
                StateChange::TimeStateChange(state) =>
                    handle_state_change_embassy!(self, state, TimeStateResponse, MessageType::TimeStateResponse),

                #[cfg(feature = "event")]
                StateChange::EventStateChange(state) =>
                    handle_state_change_embassy!(self, state, EventResponse, MessageType::EventResponse),

                #[cfg(feature = "valve")]
                StateChange::ValveStateChange(state) =>
                    handle_state_change_embassy!(self, state, ValveStateResponse, MessageType::ValveStateResponse),

                #[cfg(feature = "datetime")]
                StateChange::DateTimeStateChange(state) =>
                    handle_state_change_embassy!(self, state, DateTimeStateResponse, MessageType::DateTimeStateResponse),

                #[cfg(feature = "climate")]
                StateChange::ClimateStateChange(state) =>
                    handle_state_change_embassy!(self, state, ClimateStateResponse, MessageType::ClimateStateResponse),
            }
        }
    }
}
