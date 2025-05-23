extern crate alloc;

use crate::api::{*};
use anyhow::Result;
use femtopb::EnumValue::Known;
use femtopb::UnknownFields;

pub mod api;
pub mod metadata;
#[cfg(feature = "embassy_net")]
mod embassy_net;
pub mod std;
pub mod server;

#[derive(Default)]
pub struct DeviceConfig<'a> {
    pub name: &'a str,
    pub password: Option<&'a str>,
    pub mac_address: &'a str,
    pub esphome_version: &'a str,
    pub compilation_time: &'a str,
    pub model: &'a str,
    pub has_deep_sleep: bool,
    pub project_name: &'a str,
    pub project_version: &'a str,
    pub webserver_port: u32,
    pub legacy_bluetooth_proxy_version: u32,
    pub bluetooth_proxy_feature_flags: u32,
    pub manufacturer: &'a str,
    pub friendly_name: &'a str,
    pub legacy_voice_assistant_version: u32,
    pub voice_assistant_feature_flags: u32,
    pub suggested_area: &'a str,
    pub bluetooth_mac_address: &'a str,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BinarySensorState {
    pub key: u32,
    pub state: bool,
    pub missing_state: bool,
}

impl<'a> Into<BinarySensorStateResponse<'a>> for BinarySensorState {
    fn into(self) -> BinarySensorStateResponse<'a> {
        BinarySensorStateResponse {
            key: self.key,
            state: self.state,
            missing_state: self.missing_state,
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CoverState {
    pub key: u32,
    pub position: f32,
    pub tilt: f32,
    pub current_operation: CoverOperation,
}

impl<'a> Into<CoverStateResponse<'a>> for CoverState {
    fn into(self) -> CoverStateResponse<'a> {
        CoverStateResponse {
            key: self.key,
            position: self.position,
            legacy_state: if self.position > 0.0 {
                Known(LegacyCoverState::Open)
            } else {
                Known(LegacyCoverState::Closed)
            },
            tilt: self.tilt,
            current_operation: Known(self.current_operation),
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FanState<'a> {
    pub key: u32,
    pub state: bool,
    pub oscillating: bool,
    pub speed_level: i32,
    pub direction: FanDirection,
    pub preset_mode: &'a str,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LightState<'a> {
    pub key: u32,
    pub state: bool,
    pub brightness: f32,
    pub color_mode: ColorMode,
    pub color_brightness: f32,
    pub rgb: RgbColor,
    pub white: f32,
    pub color_temperature: f32,
    pub cold_white: f32,
    pub warm_white: f32,
    pub effect: &'a str,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SensorState {
    pub key: u32,
    pub state: f32,
    pub missing_state: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SwitchState {
    pub key: u32,
    pub state: bool,
}

impl<'a> Into<SwitchStateResponse<'a>> for SwitchState {
    fn into(self) -> SwitchStateResponse<'a> {
        SwitchStateResponse {
            key: self.key,
            state: self.state,
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TextSensorState<'a> {
    pub key: u32,
    pub state: &'a str,
    pub missing_state: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StateChange<'a> {
    BinarySensorChange(BinarySensorState),
    CoverChange(CoverState),
    FanStateChange(FanState<'a>),
    LightStateChange(LightState<'a>),
    SensorStateChange(SensorState),
    TextSensorStateChange(TextSensorState<'a>),
    SwitchStateChange(SwitchState),
}

#[derive(Clone, Copy, PartialEq)]
pub struct SwitchCommandData {
    pub key: u32,
    pub state: bool,
}

impl From<SwitchCommandRequest<'_>> for SwitchCommandData {
    fn from(request: SwitchCommandRequest) -> Self {
        SwitchCommandData {
            key: request.key,
            state: request.state,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RgbColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Clone, PartialEq)]
pub struct LightCommandData {
    pub key: u32,
    pub state: Option<bool>,
    pub brightness: Option<f32>,
    pub color_mode: Option<ColorMode>,
    pub color_brightness: Option<f32>,
    pub rgb: Option<RgbColor>,
    pub white: Option<f32>,
    pub color_temperature: Option<f32>,
    pub cold_white: Option<f32>,
    pub warm_white: Option<f32>,
    pub transition_length: Option<u32>,
    pub flash_length: Option<u32>,
    pub effect: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct CoverCommandData {
    pub key: u32,
    pub position: Option<f32>,
    pub tilt: Option<f32>,
    pub stop: bool,
}

#[derive(Clone, PartialEq)]
pub struct FanCommandData {
    pub key: u32,
    pub state: Option<bool>,
    pub oscillating: Option<bool>,
    pub direction: Option<FanDirection>,
    pub speed_level: Option<i32>,
    pub preset_mode: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct ClimateCommandData {
    pub key: u32,
    pub mode: Option<ClimateMode>,
    pub target_temperature: Option<f32>,
    pub target_temperature_low: Option<f32>,
    pub target_temperature_high: Option<f32>,
    pub fan_mode: Option<ClimateFanMode>,
    pub swing_mode: Option<ClimateSwingMode>,
    pub custom_fan_mode: Option<String>,
    pub preset: Option<ClimatePreset>,
    pub custom_preset: Option<String>,
    pub target_humidity: Option<f32>,
}

#[derive(Clone, PartialEq)]
pub struct NumberCommandData {
    pub key: u32,
    pub state: f32,
}

#[derive(Clone, PartialEq)]
pub struct SelectCommandData {
    pub key: u32,
    pub state: String,
}

#[derive(Clone, PartialEq)]
pub struct SirenCommandData {
    pub key: u32,
    pub state: Option<bool>,
    pub tone: Option<String>,
    pub duration: Option<u32>,
    pub volume: Option<f32>,
}

#[derive(Clone, PartialEq)]
pub struct LockCommandData {
    pub key: u32,
    pub command: LockCommand,
    pub code: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct ButtonCommandData {
    pub key: u32,
}

#[derive(Clone, PartialEq)]
pub struct AlarmControlPanelCommandData {
    pub key: u32,
    pub command: AlarmControlPanelStateCommand,
    pub code: String,
}

#[derive(Clone, PartialEq)]
pub struct TextCommandData {
    pub key: u32,
    pub state: String,
}

#[derive(Clone, PartialEq)]
pub struct DateCommandData {
    pub key: u32,
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

#[derive(Clone, PartialEq)]
pub struct TimeCommandData {
    pub key: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

#[derive(Clone, PartialEq)]
pub struct ValveCommandData {
    pub key: u32,
    pub position: Option<f32>,
    pub stop: bool,
}

#[derive(Clone, PartialEq)]
pub struct DateTimeCommandData {
    pub key: u32,
    pub epoch_seconds: u32,
}

#[derive(Clone, PartialEq)]
pub enum Command {
    CoverCommand(CoverCommandData),
    FanCommand(FanCommandData),
    LightCommand(LightCommandData),
    SwitchCommand(SwitchCommandData),
    ClimateCommand(ClimateCommandData),
    NumberCommand(NumberCommandData),
    SelectCommand(SelectCommandData),
    SirenCommand(SirenCommandData),
    LockCommand(LockCommandData),
    ButtonCommand(ButtonCommandData),
    AlarmControlPanelCommand(AlarmControlPanelCommandData),
    TextCommand(TextCommandData),
    DateCommand(DateCommandData),
    TimeCommand(TimeCommandData),
    ValveCommand(ValveCommandData),
    DateTimeCommand(DateTimeCommandData),
}

pub enum EntityConfig<'a> {
    BinarySensor(BinarySensorConfig<'a>),
    Switch(SwitchConfig<'a>),
}

pub struct BinarySensorConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub device_class: &'a str,
    pub is_status_binary_sensor: bool,
    pub disabled_by_default: bool,
    pub icon: &'a str,
    pub entity_category: EntityCategory,
}

impl<'a> Into<ListEntitiesBinarySensorResponse<'a>> for &BinarySensorConfig<'a> {
    fn into(self) -> ListEntitiesBinarySensorResponse<'a> {
        ListEntitiesBinarySensorResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            device_class: self.device_class,
            is_status_binary_sensor: self.is_status_binary_sensor,
            disabled_by_default: self.disabled_by_default,
            icon: self.icon,
            entity_category: Known(self.entity_category),
            unknown_fields: UnknownFields::default(),
        }
    }
}

pub struct SwitchConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub assumed_state: bool,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
    pub device_class: &'a str,
}

impl<'a> Into<ListEntitiesSwitchResponse<'a>> for &SwitchConfig<'a> {
    fn into(self) -> ListEntitiesSwitchResponse<'a> {
        ListEntitiesSwitchResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            icon: self.icon,
            assumed_state: self.assumed_state,
            disabled_by_default: self.disabled_by_default,
            entity_category: Known(self.entity_category),
            device_class: self.device_class,
            unknown_fields: UnknownFields::default(),
        }
    }
}

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

