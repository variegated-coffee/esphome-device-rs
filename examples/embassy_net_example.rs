// Embassy-net example for esphome-device
// This example demonstrates how to use esphome-device with embassy-net
//
// Note: This requires embassy-executor and a network stack setup.
// The example uses a simulated setup for demonstration purposes.

#![cfg(feature = "embassy_net")]
#![no_std]
#![no_main]

extern crate alloc;

use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};
use esphome_device::{BinarySensorState, ClientEvent, Command, DeviceConfig, EntityConfig, StateChange};
use esphome_device::api::EntityCategory;
use esphome_device::entity_type::binary_sensor::BinarySensorConfig;
use esphome_device::entity_type::switch::{SwitchConfig, SwitchState};
use esphome_device::embassy_net::server::{EspHomeConnection, EspHomeServer};

// Channel capacities
const STATE_CHANNEL_CAPACITY: usize = 10;
const EVENT_CHANNEL_CAPACITY: usize = 10;

// Static channels for state changes and events
static STATE_CHANNEL: Channel<NoopRawMutex, StateChange<'static>, STATE_CHANNEL_CAPACITY> = Channel::new();
static EVENT_CHANNEL: Channel<NoopRawMutex, ClientEvent, EVENT_CHANNEL_CAPACITY> = Channel::new();

static ENTITY_CONFIGS: &[EntityConfig] = &[
    EntityConfig::Switch(SwitchConfig {
        object_id: "relay",
        key: 1,
        name: "Relay Switch",
        unique_id: "relay_switch_001",
        icon: "mdi:power-socket",
        assumed_state: false,
        disabled_by_default: false,
        entity_category: EntityCategory::None,
        device_class: "",
    }),
    EntityConfig::BinarySensor(BinarySensorConfig {
        object_id: "motion",
        key: 2,
        name: "Motion Sensor",
        unique_id: "motion_sensor_001",
        device_class: "motion",
        is_status_binary_sensor: false,
        disabled_by_default: false,
        icon: "mdi:motion-sensor",
        entity_category: EntityCategory::None,
    }),
];

static DEVICE_CONFIG: DeviceConfig = DeviceConfig {
    name: "ESP32 Device",
    password: None,
    mac_address: "AA:BB:CC:DD:EE:FF",
    esphome_version: "2024.11.0",
    compilation_time: "Nov 17 2024, 00:00:00",
    model: "ESP32-DevKit",
    has_deep_sleep: false,
    project_name: "esphome-device-rs",
    project_version: "0.1.0",
    webserver_port: 80,
    legacy_bluetooth_proxy_version: 0,
    bluetooth_proxy_feature_flags: 0,
    manufacturer: "Espressif",
    friendly_name: "ESP32 Test Device",
    legacy_voice_assistant_version: 0,
    voice_assistant_feature_flags: 0,
    suggested_area: "Office",
    bluetooth_mac_address: "",
};

#[embassy_executor::task]
async fn handle_client_events() {
    let event_receiver = EVENT_CHANNEL.receiver();
    let state_sender = STATE_CHANNEL.sender();

    loop {
        let event = event_receiver.receive().await;

        match event {
            ClientEvent::Connected => {
                log::info!("Client connected");
            }
            ClientEvent::Disconnected => {
                log::info!("Client disconnected");
            }
            ClientEvent::CommandReceived(command) => {
                log::info!("Received command: {:?}", command);

                // Handle commands and send state updates
                match command {
                    Command::SwitchCommand(cmd) => {
                        // Update switch state
                        let new_state = StateChange::SwitchStateChange(SwitchState {
                            key: cmd.key,
                            state: cmd.state,
                        });
                        state_sender.send(new_state).await;
                    }
                    _ => {
                        log::warn!("Unhandled command type");
                    }
                }
            }
            ClientEvent::SubscribedToStates => {
                log::info!("Client subscribed to states");

                // Send initial states
                state_sender.send(StateChange::SwitchStateChange(SwitchState {
                    key: 1,
                    state: false,
                })).await;

                state_sender.send(StateChange::BinarySensorChange(BinarySensorState {
                    key: 2,
                    state: false,
                    missing_state: false,
                })).await;
            }
            ClientEvent::SubscribedToLogs => {
                log::info!("Client subscribed to logs");
            }
            ClientEvent::ReceivedTime(timestamp) => {
                log::info!("Received time from client: {}", timestamp);
            }
        }
    }
}

#[embassy_executor::task]
async fn simulate_sensor_updates() {
    let state_sender = STATE_CHANNEL.sender();
    let mut motion_state = false;

    loop {
        Timer::after(Duration::from_secs(5)).await;

        // Toggle motion sensor state
        motion_state = !motion_state;

        let state = StateChange::BinarySensorChange(BinarySensorState {
            key: 2,
            state: motion_state,
            missing_state: false,
        });

        state_sender.send(state).await;
        log::info!("Motion sensor state changed to: {}", motion_state);
    }
}

#[embassy_executor::task]
async fn esphome_server_task(
    mut socket: TcpSocket<'static>,
) {
    log::info!("ESPHome server task started");

    loop {
        log::info!("Waiting for connection on port 6053...");

        if let Err(e) = socket.accept(6053).await {
            log::error!("Accept error: {:?}", e);
            Timer::after(Duration::from_secs(1)).await;
            continue;
        }

        log::info!("Client connected from remote");

        let (mut reader, mut writer) = socket.split();

        // Create connection
        let connection = EspHomeConnection::new(&mut reader, &mut writer);

        // Create server
        let server = EspHomeServer::<'_, '_, '_, STATE_CHANNEL_CAPACITY, EVENT_CHANNEL_CAPACITY>::new(
            &connection,
            &DEVICE_CONFIG,
            ENTITY_CONFIGS,
            &STATE_CHANNEL.receiver(),
            &EVENT_CHANNEL.sender(),
        );

        // Run both loops using select
        let socket_loop = server.run_socket_loop();
        let channel_loop = server.run_channel_loop();

        // Use embassy_futures::select to run both concurrently
        match embassy_futures::select::select(socket_loop, channel_loop).await {
            embassy_futures::select::Either::First(result) => {
                if let Err(e) = result {
                    log::error!("Socket loop error: {:?}", e);
                }
            }
            embassy_futures::select::Either::Second(result) => {
                if let Err(e) = result {
                    log::error!("Channel loop error: {:?}", e);
                }
            }
        }

        log::info!("Client disconnected, waiting for new connection");
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize logging
    log::info!("ESPHome device starting...");

    // TODO: Initialize your network stack here
    // Example:
    // let stack = create_network_stack();
    // stack.run().await;

    // Spawn event handler task
    spawner.spawn(handle_client_events()).unwrap();

    // Spawn sensor simulation task
    spawner.spawn(simulate_sensor_updates()).unwrap();

    // TODO: Create TCP socket with your network stack
    // Example:
    // let mut rx_buffer = [0; 4096];
    // let mut tx_buffer = [0; 4096];
    // let socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

    // spawner.spawn(esphome_server_task(socket)).unwrap();

    log::info!("All tasks spawned");

    // Main loop
    loop {
        Timer::after(Duration::from_secs(60)).await;
        log::info!("System running...");
    }
}

// Platform-specific panic handler and allocator would go here
// This depends on your target platform (ESP32, STM32, etc.)
