// Embassy-net API usage example for esphome-device
//
// This example demonstrates a working embassy-net integration.
// It creates a TCP server using embassy-net and integrates with EspHomeServer.
//
// To run this example:
// 1. Install dependencies: sudo apt-get install uml-utilities
// 2. Setup TUN device: sudo ip tuntap add name tap0 mode tap
// 3. Configure IP: sudo ip addr add 192.168.69.1/24 dev tap0
// 4. Bring up interface: sudo ip link set tap0 up
// 5. Run: sudo -E cargo run --example embassy_net_example --features embassy_net
// 6. Connect with ESPHome client to 192.168.69.2:6053

#![cfg(feature = "embassy_net")]

use embassy_executor::Spawner;
use embassy_net::{Stack, StackResources, Runner};
use embassy_net::tcp::TcpSocket;
use embassy_time::{Duration, Timer};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;

use esphome_device::{BinarySensorState, ClientEvent, Command, DeviceConfig, EntityConfig, StateChange};
use esphome_device::api::EntityCategory;
use esphome_device::entity_type::binary_sensor::BinarySensorConfig;
use esphome_device::entity_type::switch::{SwitchConfig, SwitchState};
use esphome_device::embassy_net::server::{EspHomeConnection, EspHomeServer};

use smoltcp::wire::{Ipv4Address, Ipv4Cidr};

// Entity configuration
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

// Static channels for communication
static STATE_CHANNEL: Channel<CriticalSectionRawMutex, StateChange, 10> = Channel::new();
static EVENT_CHANNEL: Channel<CriticalSectionRawMutex, ClientEvent, 10> = Channel::new();

// TUN/TAP device wrapper
struct TunTapDevice {
    fd: i32,
}

impl TunTapDevice {
    fn new(name: &str) -> std::io::Result<Self> {
        const TUNSETIFF: libc::c_ulong = 0x400454ca;
        const IFF_TAP: libc::c_short = 0x0002;
        const IFF_NO_PI: libc::c_short = 0x1000;

        unsafe {
            let fd = libc::open(b"/dev/net/tun\0".as_ptr() as *const _, libc::O_RDWR);
            if fd < 0 {
                return Err(std::io::Error::last_os_error());
            }

            let mut ifr = libc::ifreq {
                ifr_name: [0; libc::IF_NAMESIZE],
                ifr_ifru: libc::__c_anonymous_ifr_ifru {
                    ifru_flags: IFF_TAP | IFF_NO_PI,
                },
            };

            let name_bytes = name.as_bytes();
            ifr.ifr_name[..name_bytes.len()].copy_from_slice(
                &std::mem::transmute::<&[u8], &[i8]>(name_bytes)[..name_bytes.len()]
            );

            if libc::ioctl(fd, TUNSETIFF as _, &mut ifr) < 0 {
                libc::close(fd);
                return Err(std::io::Error::last_os_error());
            }

            Ok(Self { fd })
        }
    }
}

impl Drop for TunTapDevice {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}

impl embassy_net::driver::Driver for TunTapDevice {
    type RxToken<'a> = RxToken where Self: 'a;
    type TxToken<'a> = TxToken where Self: 'a;

    fn receive(&mut self, _cx: &mut core::task::Context) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        // Simple polling implementation
        let mut buf = [0u8; 1536];
        match unsafe {
            libc::read(self.fd, buf.as_mut_ptr() as *mut _, buf.len())
        } {
            n if n > 0 => {
                Some((
                    RxToken { buffer: buf[..n as usize].to_vec() },
                    TxToken { fd: self.fd },
                ))
            }
            _ => None,
        }
    }

    fn transmit(&mut self, _cx: &mut core::task::Context) -> Option<Self::TxToken<'_>> {
        Some(TxToken { fd: self.fd })
    }

    fn link_state(&mut self, _cx: &mut core::task::Context) -> embassy_net::driver::LinkState {
        embassy_net::driver::LinkState::Up
    }

    fn capabilities(&self) -> embassy_net::driver::Capabilities {
        let mut caps = embassy_net::driver::Capabilities::default();
        caps.max_transmission_unit = 1500;
        caps.max_burst_size = Some(1);
        caps
    }

    fn hardware_address(&self) -> embassy_net::driver::HardwareAddress {
        embassy_net::driver::HardwareAddress::Ethernet([0x02, 0x00, 0x00, 0x00, 0x00, 0x01])
    }
}

struct RxToken {
    buffer: Vec<u8>,
}

impl embassy_net::driver::RxToken for RxToken {
    fn consume<R, F>(mut self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        f(&mut self.buffer)
    }
}

struct TxToken {
    fd: i32,
}

impl embassy_net::driver::TxToken for TxToken {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buf = vec![0u8; len];
        let result = f(&mut buf);
        unsafe {
            libc::write(self.fd, buf.as_ptr() as *const _, len);
        }
        result
    }
}

// Event handler task
#[embassy_executor::task]
async fn handle_events() {
    log::info!("Event handler task started");

    loop {
        match EVENT_CHANNEL.receiver().receive().await {
            ClientEvent::CommandReceived(Command::SwitchCommand(cmd)) => {
                log::info!("Received switch command for key {}: {}", cmd.key, cmd.state);

                // Echo back the state change
                STATE_CHANNEL.sender().send(
                    StateChange::SwitchStateChange(SwitchState {
                        key: cmd.key,
                        state: cmd.state,
                    })
                ).await;
            }
            ClientEvent::SubscribedToStates => {
                log::info!("Client subscribed to states, sending initial states");

                // Send initial state for all entities
                STATE_CHANNEL.sender().send(
                    StateChange::SwitchStateChange(SwitchState {
                        key: 1,
                        state: false,
                    })
                ).await;

                STATE_CHANNEL.sender().send(
                    StateChange::BinarySensorChange(BinarySensorState {
                        key: 2,
                        state: false,
                        missing_state: false,
                    })
                ).await;
            }
            _ => {
                log::debug!("Received other event");
            }
        }
    }
}

// Network stack task
#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, TunTapDevice>) -> ! {
    runner.run().await
}

// ESPHome server task
#[embassy_executor::task]
async fn esphome_server(stack: &'static Stack<'static>) {
    log::info!("ESPHome server task started");

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    loop {
        let mut socket = TcpSocket::new(stack.clone(), &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        log::info!("Listening on TCP port 6053...");
        if let Err(e) = socket.accept(6053).await {
            log::warn!("Accept error: {:?}", e);
            Timer::after(Duration::from_secs(1)).await;
            continue;
        }

        log::info!("Client connected!");

        // Split socket into reader and writer
        let (mut reader, mut writer) = socket.split();

        // Create connection and server
        let connection = EspHomeConnection::new(&mut reader, &mut writer);
        let state_receiver = STATE_CHANNEL.receiver();
        let event_sender = EVENT_CHANNEL.sender();
        let server: EspHomeServer<10, 10> = EspHomeServer::new(
            &connection,
            &DEVICE_CONFIG,
            ENTITY_CONFIGS,
            &state_receiver,
            &event_sender,
        );

        // Run both loops concurrently
        log::info!("Running ESPHome protocol loops");
        let result = embassy_futures::select::select(
            server.run_socket_loop(),
            server.run_channel_loop()
        ).await;

        match result {
            embassy_futures::select::Either::First(res) => {
                log::info!("Socket loop ended: {:?}", res);
            }
            embassy_futures::select::Either::Second(res) => {
                log::info!("Channel loop ended: {:?}", res);
            }
        }

        log::info!("Client disconnected");
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    log::info!("=== ESPHome Device Embassy-net Example ===");
    log::info!("Device: {} ({})", DEVICE_CONFIG.name, DEVICE_CONFIG.model);
    log::info!("Entities: {}", ENTITY_CONFIGS.len());

    // Create TUN/TAP device
    let device = match TunTapDevice::new("tap0") {
        Ok(dev) => dev,
        Err(e) => {
            log::error!("Failed to create TUN/TAP device: {}", e);
            log::error!("Make sure you have run:");
            log::error!("  sudo ip tuntap add name tap0 mode tap");
            log::error!("  sudo ip addr add 192.168.69.1/24 dev tap0");
            log::error!("  sudo ip link set tap0 up");
            return;
        }
    };

    // Create network stack
    let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
        address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 69, 2), 24),
        gateway: Some(Ipv4Address::new(192, 168, 69, 1)),
        dns_servers: Default::default(),
    });

    // Create stack resources
    let resources = Box::leak(Box::new(StackResources::<3>::new()));

    // Create network stack and runner
    let (stack, runner) = embassy_net::new(
        device,
        config,
        resources,
        embassy_time::Instant::now().as_millis() as u64,
    );

    // Leak the stack to make it 'static
    let stack: &'static Stack<'static> = Box::leak(Box::new(stack));

    log::info!("Network configured: 192.168.69.2/24");
    log::info!("Gateway: 192.168.69.1");
    log::info!("Listening on: 192.168.69.2:6053");

    // Spawn tasks
    spawner.spawn(net_task(runner).unwrap());
    spawner.spawn(handle_events().unwrap());
    spawner.spawn(esphome_server(stack).unwrap());

    log::info!("All tasks spawned, server is running");
    log::info!("Connect ESPHome client to 192.168.69.2:6053");
}
