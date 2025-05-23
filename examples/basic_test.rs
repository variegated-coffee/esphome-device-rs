use std::net::TcpListener;
use std::ops::DerefMut;
use anyhow::{anyhow, format_err, Result};
use esphome_device::{api::{
    HelloRequest, HelloResponse, PingRequest, PingResponse, DeviceInfoRequest, DeviceInfoResponse,
    ConnectRequest, ConnectResponse, DisconnectRequest, DisconnectResponse
}, ApiConnection, BinarySensorConfig, BinarySensorState, Command, DeviceConfig, EntityConfig, StateChange, SwitchConfig, SwitchState};
use async_std::task;
use async_io::Async;
use async_std::channel::{Sender, Receiver};
use femtopb::EnumValue::Known;
use femtopb::{EnumValue, Message, UnknownFields};
use futures::join;
use log::log;
use esphome_device::api::{AlarmControlPanelCommandRequest, BinarySensorStateResponse, BluetoothConnectionsFreeResponse, BluetoothDeviceRequest, BluetoothGattGetServicesRequest, BluetoothGattNotifyRequest, BluetoothGattReadDescriptorRequest, BluetoothGattReadRequest, BluetoothGattWriteDescriptorRequest, BluetoothGattWriteRequest, BluetoothScannerSetModeRequest, ButtonCommandRequest, CameraImageRequest, ClimateCommandRequest, ColorMode, CoverCommandRequest, CoverOperation, CoverStateResponse, DateCommandRequest, DateTimeCommandRequest, EntityCategory, ExecuteServiceRequest, FanCommandRequest, FanDirection, GetTimeRequest, GetTimeResponse, LegacyCoverState, LightCommandRequest, ListEntitiesBinarySensorResponse, ListEntitiesDoneResponse, ListEntitiesRequest, LockCommandRequest, LogLevel, MediaPlayerCommandRequest, NoiseEncryptionSetKeyRequest, NoiseEncryptionSetKeyResponse, NumberCommandRequest, SelectCommandRequest, SirenCommandRequest, SubscribeBluetoothConnectionsFreeRequest, SubscribeBluetoothLeAdvertisementsRequest, SubscribeHomeAssistantStatesRequest, SubscribeHomeassistantServicesRequest, SubscribeLogsRequest, SubscribeLogsResponse, SubscribeStatesRequest, SubscribeVoiceAssistantRequest, SwitchCommandRequest, TextCommandRequest, TimeCommandRequest, UnsubscribeBluetoothLeAdvertisementsRequest, UpdateCommandRequest, ValveCommandRequest, VoiceAssistantConfigurationRequest, VoiceAssistantConfigurationResponse, VoiceAssistantSetConfiguration};
use esphome_device::std::server::{EspHomeConnection, EspHomeServer};
use esphome_device::server::ConnectionStatus;
use esphome_device::metadata::MessageType;

fn main() -> Result<()> {
    use env_logger::Env;

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Create and run the server
    task::block_on(async {
        println!("Starting ESPHome server on 0.0.0.0:6053");

        if let Err(e) = run("0.0.0.0", 6053).await {
            eprintln!("Server error: {}", e);
        }
        
        Ok(())
    })
}

static ENTITY_CONFIGS: &[EntityConfig] = &[
    EntityConfig::Switch(SwitchConfig {
        object_id: "switch",
        key: 2,
        name: "Test Switch",
        unique_id: "test_switch",
        icon: "",
        assumed_state: false,
        disabled_by_default: false,
        entity_category: EntityCategory::Config,
        device_class: "",
    }),
    EntityConfig::BinarySensor(BinarySensorConfig {
        object_id: "binary_sensor",
        key: 1,
        name: "Test Binary Sensor",
        unique_id: "test_binary_sensor",
        device_class: "motion",
        is_status_binary_sensor: false,
        disabled_by_default: false,
        icon: "mdi:home",
        entity_category: EntityCategory::None,
    }),
];

static DEVICE_CONFIG: &DeviceConfig = &DeviceConfig {
    name: "Test Device",
    password: None,
    mac_address: "",
    esphome_version: "",
    compilation_time: "",
    model: "",
    has_deep_sleep: false,
    project_name: "",
    project_version: "",
    webserver_port: 0,
    legacy_bluetooth_proxy_version: 0,
    bluetooth_proxy_feature_flags: 0,
    manufacturer: "",
    friendly_name: "",
    legacy_voice_assistant_version: 0,
    voice_assistant_feature_flags: 0,
    suggested_area: "",
    bluetooth_mac_address: "",
};

/// Start the server
pub async fn run(address: &str, port: u16) -> Result<()> {
    let addr = format!("{}:{}", address, port);
    let listener = TcpListener::bind(&addr)
        .map_err(|e| format_err!("Failed to bind to {}: {}", addr, e))?;

    let async_listener = Async::new(listener)
        .map_err(|e| format_err!("Failed to create async listener: {}", e))?;

    let (state_change_sender, state_change_receiver) = async_std::channel::unbounded();
    let (command_sender, command_receiver) = async_std::channel::unbounded();

    async_std::task::spawn(sensor_states_task(state_change_sender, command_receiver));

    loop {
        // Accept a new connection
        let (stream, peer_addr) = async_listener.accept().await
            .map_err(|e| format_err!("Accept error: {}", e))?;

        log::info!("Accepted connection from {}", peer_addr);

        let conn = EspHomeConnection::new(stream);
        let mut server = EspHomeServer::new(conn, DEVICE_CONFIG, ENTITY_CONFIGS, state_change_receiver.clone(), command_sender.clone());
        let server_task = async_std::task::spawn(async move {
            if let Err(e) = server.run().await {
                log::error!("Server error: {}", e);
            }
        });
    }
}

pub async fn sensor_states_task(sender: Sender<StateChange<'_>>, command_receiver: Receiver<Command>) {
    let mut state = false;
    loop {
        // Wait for a command 
        let command = command_receiver.recv().await.unwrap();

        match command {
            Command::SwitchCommand(data) => {
                log::info!("Switch command received, setting states to {}", data.state);
                state = data.state;
            }
            _ => {}
        }

        // Simulate sending state changes
        let _ = sender.send(StateChange::BinarySensorChange(BinarySensorState {
            key: 1,
            state,
            missing_state: false
        })).await;

        let _ = sender.send(StateChange::SwitchStateChange(SwitchState {
            key: 2,
            state,
        })).await;
    }
}