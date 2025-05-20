extern crate alloc;

use crate::api::{*};
use anyhow::Result;

pub mod api;
mod metadata;
#[cfg(feature = "embassy_net")]
mod embassy_net;
pub mod std;

#[async_trait::async_trait]
pub trait ApiConnection {
    async fn receive_states_task(&self) -> Result<()>;
    
    // Basic connection methods
    async fn hello<'a>(&self, request: HelloRequest<'a>) -> Result<HelloResponse>;
    async fn device_info<'a>(&self, request: DeviceInfoRequest<'a>) -> Result<DeviceInfoResponse>;

    async fn connect<'a>(&self, request: ConnectRequest<'a>) -> Result<ConnectResponse>;
    async fn disconnect<'a>(&self, request: DisconnectRequest<'a>) -> Result<DisconnectResponse>;
    async fn ping<'a>(&self, request: PingRequest<'a>) -> Result<PingResponse>;
    /*
        // Entity listing
        async fn list_entities(&self, request: ListEntitiesRequest) -> Result<()>;
    
    */
        // Subscription methods
        async fn subscribe_states<'a>(&self, request: SubscribeStatesRequest<'a>) -> Result<()>;
        async fn subscribe_logs<'a>(&self, request: SubscribeLogsRequest<'a>) -> Result<SubscribeLogsResponse>;
        async fn subscribe_homeassistant_services<'a>(&self, request: SubscribeHomeassistantServicesRequest<'a>) -> Result<()>;
        async fn subscribe_home_assistant_states<'a>(&self, request: SubscribeHomeAssistantStatesRequest<'a>) -> Result<()>;
    /*
        // Time methods
        async fn get_time(&self, request: GetTimeRequest) -> Result<GetTimeResponse>;
    
        // Service execution
        async fn execute_service(&self, request: ExecuteServiceRequest) -> Result<()>;
    
        // Encryption
        async fn noise_encryption_set_key(&self, request: NoiseEncryptionSetKeyRequest) -> Result<NoiseEncryptionSetKeyResponse>;
    
        // Component control methods
        async fn button_command(&self, request: ButtonCommandRequest) -> Result<()>;
        async fn camera_image(&self, request: CameraImageRequest) -> Result<()>;
        async fn climate_command(&self, request: ClimateCommandRequest) -> Result<()>;
        async fn cover_command(&self, request: CoverCommandRequest) -> Result<()>;
        async fn date_command(&self, request: DateCommandRequest) -> Result<()>;
        async fn datetime_command(&self, request: DateTimeCommandRequest) -> Result<()>;
        async fn fan_command(&self, request: FanCommandRequest) -> Result<()>;
        async fn light_command(&self, request: LightCommandRequest) -> Result<()>;
        async fn lock_command(&self, request: LockCommandRequest) -> Result<()>;
        async fn media_player_command(&self, request: MediaPlayerCommandRequest) -> Result<()>;
        async fn number_command(&self, request: NumberCommandRequest) -> Result<()>;
        async fn select_command(&self, request: SelectCommandRequest) -> Result<()>;
        async fn siren_command(&self, request: SirenCommandRequest) -> Result<()>;
        async fn switch_command(&self, request: SwitchCommandRequest) -> Result<()>;
        async fn text_command(&self, request: TextCommandRequest) -> Result<()>;
        async fn time_command(&self, request: TimeCommandRequest) -> Result<()>;
        async fn update_command(&self, request: UpdateCommandRequest) -> Result<()>;
        async fn valve_command(&self, request: ValveCommandRequest) -> Result<()>;
        async fn alarm_control_panel_command(&self, request: AlarmControlPanelCommandRequest) -> Result<()>;
    
        // Bluetooth methods
        async fn subscribe_bluetooth_le_advertisements(&self, request: SubscribeBluetoothLeAdvertisementsRequest) -> Result<()>;
        async fn bluetooth_device_request(&self, request: BluetoothDeviceRequest) -> Result<()>;
        async fn bluetooth_gatt_get_services(&self, request: BluetoothGattGetServicesRequest) -> Result<()>;
        async fn bluetooth_gatt_read(&self, request: BluetoothGattReadRequest) -> Result<()>;
        async fn bluetooth_gatt_write(&self, request: BluetoothGattWriteRequest) -> Result<()>;
        async fn bluetooth_gatt_read_descriptor(&self, request: BluetoothGattReadDescriptorRequest) -> Result<()>;
        async fn bluetooth_gatt_write_descriptor(&self, request: BluetoothGattWriteDescriptorRequest) -> Result<()>;
        async fn bluetooth_gatt_notify(&self, request: BluetoothGattNotifyRequest) -> Result<()>;
        async fn subscribe_bluetooth_connections_free(&self, request: SubscribeBluetoothConnectionsFreeRequest) -> Result<BluetoothConnectionsFreeResponse>;
        async fn unsubscribe_bluetooth_le_advertisements(&self, request: UnsubscribeBluetoothLeAdvertisementsRequest) -> Result<()>;
        async fn bluetooth_scanner_set_mode(&self, request: BluetoothScannerSetModeRequest) -> Result<()>;
    
        // Voice assistant methods
        async fn subscribe_voice_assistant(&self, request: SubscribeVoiceAssistantRequest) -> Result<()>;
        async fn voice_assistant_get_configuration(&self, request: VoiceAssistantConfigurationRequest) -> Result<VoiceAssistantConfigurationResponse>;
        async fn voice_assistant_set_configuration(&self, request: VoiceAssistantSetConfiguration) -> Result<()>;*/
}

