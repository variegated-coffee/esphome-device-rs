use std::net::TcpListener;
use anyhow::{format_err, Result};
use esphome_device::{
    api::{
        HelloRequest, HelloResponse, PingRequest, PingResponse, DeviceInfoRequest, DeviceInfoResponse,
        ConnectRequest, ConnectResponse, DisconnectRequest, DisconnectResponse
    },
    ApiConnection,
};
use async_std::task;
use std::sync::Arc;
use std::time::Duration;
use async_io::Async;
use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use esphome_device::api::{AlarmControlPanelCommandRequest, BluetoothConnectionsFreeResponse, BluetoothDeviceRequest, BluetoothGattGetServicesRequest, BluetoothGattNotifyRequest, BluetoothGattReadDescriptorRequest, BluetoothGattReadRequest, BluetoothGattWriteDescriptorRequest, BluetoothGattWriteRequest, BluetoothScannerSetModeRequest, ButtonCommandRequest, CameraImageRequest, ClimateCommandRequest, CoverCommandRequest, DateCommandRequest, DateTimeCommandRequest, ExecuteServiceRequest, FanCommandRequest, GetTimeRequest, GetTimeResponse, LightCommandRequest, ListEntitiesRequest, LockCommandRequest, LogLevel, MediaPlayerCommandRequest, NoiseEncryptionSetKeyRequest, NoiseEncryptionSetKeyResponse, NumberCommandRequest, SelectCommandRequest, SirenCommandRequest, SubscribeBluetoothConnectionsFreeRequest, SubscribeBluetoothLeAdvertisementsRequest, SubscribeHomeAssistantStatesRequest, SubscribeHomeassistantServicesRequest, SubscribeLogsRequest, SubscribeLogsResponse, SubscribeStatesRequest, SubscribeVoiceAssistantRequest, SwitchCommandRequest, TextCommandRequest, TimeCommandRequest, UnsubscribeBluetoothLeAdvertisementsRequest, UpdateCommandRequest, ValveCommandRequest, VoiceAssistantConfigurationRequest, VoiceAssistantConfigurationResponse, VoiceAssistantSetConfiguration};
use esphome_device::std::server::EspHomeServer;

/// Basic ApiConnection implementation that responds to hello and ping messages
#[derive(Clone)]
struct BasicApiService {
    name: String,
    version: String,
}

#[async_trait::async_trait]
impl ApiConnection for BasicApiService {
    async fn receive_states_task(&self) -> Result<()> {
        loop {
            // Simulate receiving states
            println!("Receiving states...");
            task::sleep(Duration::from_secs(10)).await;
            
            
        }
    }

    async fn hello<'a>(&self, request: HelloRequest<'a>) -> Result<HelloResponse<'_>> {
        println!("Received hello from client");
        
        // Create a simple hello response
        Ok(HelloResponse {
            api_version_major: 1,
            api_version_minor: 5,
            server_info: "ESPHome Device Server".into(),
            name: "Hello from ESPHome".into(),
            unknown_fields: UnknownFields::default(),
        })
    }

    async fn device_info<'a>(&self, request: DeviceInfoRequest<'a>) -> Result<DeviceInfoResponse> {
        println!("Received device info request");
        
        // Create a simple device info response
        Ok(DeviceInfoResponse {
            uses_password: false,
            name: "Some Name".into(),
            mac_address: "00:00:00:00:00:00".into(),
            esphome_version: "1.0.0".into(),
            compilation_time: "2023-10-01T00:00:00Z".into(),
            model: "Some Model".into(),
            has_deep_sleep: false,
            project_name: "Some Project".into(),
            project_version: "1.0.0".into(),
            webserver_port: 80,
            legacy_bluetooth_proxy_version: 0,
            bluetooth_proxy_feature_flags: 0,
            manufacturer: "Some Manufacturer".into(),
            friendly_name: "Some Friendly Name".into(),
            legacy_voice_assistant_version: 0,
            voice_assistant_feature_flags: 0,
            suggested_area: "Nowhere".into(),
            bluetooth_mac_address: "00:00:00:00:00:00".into(),
            api_encryption_supported: false,
            unknown_fields: Default::default(),
        })
    }

    async fn connect<'a>(&self, request: ConnectRequest<'a>) -> Result<ConnectResponse> {
        println!("Received connect request, password: {}", request.password);
        
        // Create a simple connect response
        Ok(ConnectResponse {
            invalid_password: false,
            unknown_fields: UnknownFields::default(),
        })
    }

    async fn disconnect<'a>(&self, request: DisconnectRequest<'a>) -> Result<DisconnectResponse> {
        println!("Received disconnect request");
        
        // Create a simple disconnect response
        Ok(DisconnectResponse {
            unknown_fields: UnknownFields::default(),
        })
    }

    async fn ping<'a>(&self, request: PingRequest<'a>) -> Result<PingResponse> {
        println!("Received ping request");
        
        // Create a simple ping response
        Ok(PingResponse {
            unknown_fields: UnknownFields::default(),
        })
    }

    async fn subscribe_states<'a>(&self, request: SubscribeStatesRequest<'a>) -> Result<()> {
        println!("Received subscribe states request");
        Ok(())
    }

    async fn subscribe_logs<'a>(&self, request: SubscribeLogsRequest<'a>) -> Result<SubscribeLogsResponse> {
        println!("Received subscribe logs request");
        
        // Create a simple subscribe logs response
        Ok(SubscribeLogsResponse {
            level: Known(LogLevel::Info),
            message: &[0xDE,0xAD,0xBE,0xEF],
            send_failed: false,
            unknown_fields: UnknownFields::default(),
        })
    }

    async fn subscribe_homeassistant_services<'a>(&self, request: SubscribeHomeassistantServicesRequest<'a>) -> Result<()> {
        println!("Received subscribe homeassistant services request");
        
        Ok(())
    }

    async fn subscribe_home_assistant_states<'a>(&self, request: SubscribeHomeAssistantStatesRequest<'a>) -> Result<()> {
        println!("Received subscribe homeassistant states request");
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init();
    
    // Create our API service
    let service = BasicApiService {
        name: "Basic Test Server".into(),
        version: "1.0.0".into(),
    };
    
    // Create and run the server
    task::block_on(async {
        println!("Starting ESPHome server on 0.0.0.0:6053");
        
        let mut server = esphome_device::std::server::EspHomeServer::new(service);
        
        if let Err(e) = run(&mut server, "0.0.0.0", 6053).await {
            eprintln!("Server error: {}", e);
        }
        
        Ok(())
    })
}

/// Start the server
pub async fn run(server: &mut EspHomeServer<BasicApiService>, address: &str, port: u16) -> Result<()> {
    let addr = format!("{}:{}", address, port);
    let listener = TcpListener::bind(&addr)
        .map_err(|e| format_err!("Failed to bind to {}: {}", addr, e))?;

    let async_listener = Async::new(listener)
        .map_err(|e| format_err!("Failed to create async listener: {}", e))?;

    loop {
        // Accept a new connection
        let (stream, peer_addr) = async_listener.accept().await
            .map_err(|e| format_err!("Accept error: {}", e))?;

        log::info!("Accepted connection from {}", peer_addr);

        let e = server.handle(stream).await
            .map_err(|e| format_err!("Connection error: {}", e));
    }
}