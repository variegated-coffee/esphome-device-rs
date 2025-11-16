use std::net::TcpListener;
use anyhow::{format_err, Result};
use esphome_device::{BinarySensorConfig, BinarySensorState, ClientEvent, Command, DeviceConfig, EntityConfig, StateChange};
use async_std::task;
use async_io::Async;
use async_std::channel::{Sender, Receiver};
use esphome_device::api::{CoverOperation, EntityCategory};
use esphome_device::entity_type::cover::CoverState;
use esphome_device::entity_type::fan::FanState;
use esphome_device::entity_type::light::LightState;
use esphome_device::entity_type::switch::{SwitchConfig, SwitchState};
use esphome_device::std::server::{EspHomeConnection};
use esphome_device::server::{EspHomeServer};

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
        let _ = async_std::task::spawn(async move {
            if let Err(e) = server.run().await {
                log::error!("Server error: {}", e);
            }
        });
    }
}

pub async fn sensor_states_task(sender: Sender<StateChange<'_>>, client_event_receiver: Receiver<ClientEvent>) {
    loop {
        // Wait for a command 
        let command = client_event_receiver.recv().await.unwrap();
        
        match command {
            ClientEvent::CommandReceived(command) => {
                sender.send(
                // Handle the command
                match command {
                    Command::SwitchCommand(command) => StateChange::SwitchStateChange(SwitchState {
                        key: command.key,
                        state: command.state,
                    }),
                    Command::CoverCommand(command) => StateChange::CoverChange(CoverState {
                        key: command.key,
                        position: command.position.unwrap_or(30.0),
                        tilt: command.tilt.unwrap_or(15.0),
                        current_operation: CoverOperation::Idle,
                    }),
                    Command::FanCommand(command) => StateChange::FanStateChange(FanState {
                        key: command.key,
                        state: command.state.unwrap_or(true),
                        oscillating: command.oscillating.unwrap_or(false),
                        speed_level: command.speed_level.unwrap_or(0),
                        direction: command.direction.unwrap_or_default(),
                        preset_mode: "",
                    }),
                    Command::LightCommand(command) => StateChange::LightStateChange(LightState {
                        key: command.key,
                        state: command.state.unwrap_or(true),
                        brightness: command.brightness.unwrap_or(100.0),
                        color_mode: command.color_mode.unwrap_or_default(),
                        color_brightness: command.color_brightness.unwrap_or(100.0),
                        rgb: command.rgb.unwrap_or_default(),
                        white: command.white.unwrap_or(100.0),
                        color_temperature: command.color_temperature.unwrap_or(5000.0),
                        cold_white: command.cold_white.unwrap_or(100.0),
                        warm_white: command.warm_white.unwrap_or(100.0),
                        effect: "",
                    }),
                    _ => {
                        log::warn!("Received unsupported command");
                        
                        StateChange::BinarySensorChange(BinarySensorState {
                            key: 1,
                            state: false,
                            missing_state: false,
                        })
                    }
                }).await.unwrap();
            }
            ClientEvent::SubscribedToStates => {
                log::info!("Client subscribed to states");
                // Send initial state updates for all entities
                for entity in ENTITY_CONFIGS {
                    let state_change = create_state_change(entity);
                    match state_change {
                        Ok(change) => {
                            let _ = sender.send(change).await;
                        }
                        Err(_) => {
                            log::error!("Failed to create state change for entity");
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn create_state_change<'a>(entity: &'a EntityConfig<'a>) -> Result<StateChange<'a>> {
        match entity {
            EntityConfig::Switch(config) => {
                let state = SwitchState {
                    key: config.key,
                    .. Default::default()
                };
                
                Ok(StateChange::SwitchStateChange(state))
            }
            EntityConfig::BinarySensor(config) => {
                let state = BinarySensorState {
                    key: config.key,
                    .. Default::default()
                };
                
                Ok(StateChange::BinarySensorChange(state))
            }
            _ => {
                Err(format_err!("Unsupported entity type for state change"))
            }
        }
}