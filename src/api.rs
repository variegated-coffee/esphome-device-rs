#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct Void<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum ApiSourceType {
    #[default]
    SourceBoth = 0,
    SourceServer = 1,
    SourceClient = 2,
}
impl ApiSourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::SourceBoth => "SOURCE_BOTH",
            Self::SourceServer => "SOURCE_SERVER",
            Self::SourceClient => "SOURCE_CLIENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SOURCE_BOTH" => Some(Self::SourceBoth),
            "SOURCE_SERVER" => Some(Self::SourceServer),
            "SOURCE_CLIENT" => Some(Self::SourceClient),
            _ => None,
        }
    }
}
/// Message sent at the beginning of each connection
/// Can only be sent by the client and only at the beginning of the connection
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct HelloRequest<'a> {
    /// Description of client (like User Agent)
    /// For example "Home Assistant"
    /// Not strictly necessary to send but nice for debugging
    /// purposes.
    #[femtopb(string, tag = 1)]
    pub client_info: &'a str,
    #[femtopb(uint32, tag = 2)]
    pub api_version_major: u32,
    #[femtopb(uint32, tag = 3)]
    pub api_version_minor: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Confirmation of successful connection request.
/// Can only be sent by the server and only at the beginning of the connection
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct HelloResponse<'a> {
    /// The version of the API to use. The _client_ (for example Home Assistant) needs to check
    /// for compatibility and if necessary adopt to an older API.
    /// Major is for breaking changes in the base protocol - a mismatch will lead to immediate disconnect_client_
    /// Minor is for breaking changes in individual messages - a mismatch will lead to a warning message
    #[femtopb(uint32, tag = 1)]
    pub api_version_major: u32,
    #[femtopb(uint32, tag = 2)]
    pub api_version_minor: u32,
    /// A string identifying the server (ESP); like client info this may be empty
    /// and only exists for debugging/logging purposes.
    /// For example "ESPHome v1.10.0 on ESP8266"
    #[femtopb(string, tag = 3)]
    pub server_info: &'a str,
    /// The name of the server (App.get_name())
    #[femtopb(string, tag = 4)]
    pub name: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Message sent at the beginning of each connection to authenticate the client
/// Can only be sent by the client and only at the beginning of the connection
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ConnectRequest<'a> {
    /// The password to log in with
    #[femtopb(string, tag = 1)]
    pub password: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Confirmation of successful connection. After this the connection is available for all traffic.
/// Can only be sent by the server and only at the beginning of the connection
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct ConnectResponse<'a> {
    #[femtopb(bool, tag = 1)]
    pub invalid_password: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Request to close the connection.
/// Can be sent by both the client and server
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct DisconnectRequest<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct DisconnectResponse<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PingRequest<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PingResponse<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct DeviceInfoRequest<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct DeviceInfoResponse<'a> {
    #[femtopb(bool, tag = 1)]
    pub uses_password: bool,
    /// The name of the node, given by "App.set_name()"
    #[femtopb(string, tag = 2)]
    pub name: &'a str,
    /// The mac address of the device. For example "AC:BC:32:89:0E:A9"
    #[femtopb(string, tag = 3)]
    pub mac_address: &'a str,
    /// A string describing the ESPHome version. For example "1.10.0"
    #[femtopb(string, tag = 4)]
    pub esphome_version: &'a str,
    /// A string describing the date of compilation, this is generated by the compiler
    /// and therefore may not be in the same format all the time.
    /// If the user isn't using ESPHome, this will also not be set.
    #[femtopb(string, tag = 5)]
    pub compilation_time: &'a str,
    /// The model of the board. For example NodeMCU
    #[femtopb(string, tag = 6)]
    pub model: &'a str,
    #[femtopb(bool, tag = 7)]
    pub has_deep_sleep: bool,
    /// The esphome project details if set
    #[femtopb(string, tag = 8)]
    pub project_name: &'a str,
    #[femtopb(string, tag = 9)]
    pub project_version: &'a str,
    #[femtopb(uint32, tag = 10)]
    pub webserver_port: u32,
    #[femtopb(uint32, tag = 11)]
    pub legacy_bluetooth_proxy_version: u32,
    #[femtopb(uint32, tag = 15)]
    pub bluetooth_proxy_feature_flags: u32,
    #[femtopb(string, tag = 12)]
    pub manufacturer: &'a str,
    #[femtopb(string, tag = 13)]
    pub friendly_name: &'a str,
    #[femtopb(uint32, tag = 14)]
    pub legacy_voice_assistant_version: u32,
    #[femtopb(uint32, tag = 17)]
    pub voice_assistant_feature_flags: u32,
    #[femtopb(string, tag = 16)]
    pub suggested_area: &'a str,
    /// The Bluetooth mac address of the device. For example "AC:BC:32:89:0E:AA"
    #[femtopb(string, tag = 18)]
    pub bluetooth_mac_address: &'a str,
    /// Supports receiving and saving api encryption key
    #[femtopb(bool, tag = 19)]
    pub api_encryption_supported: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesRequest<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesDoneResponse<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct SubscribeStatesRequest<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== BINARY SENSOR ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesBinarySensorResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub device_class: &'a str,
    #[femtopb(bool, tag = 6)]
    pub is_status_binary_sensor: bool,
    #[femtopb(bool, tag = 7)]
    pub disabled_by_default: bool,
    #[femtopb(string, tag = 8)]
    pub icon: &'a str,
    #[femtopb(enumeration, tag = 9)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BinarySensorStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bool, tag = 2)]
    pub state: bool,
    /// If the binary sensor does not have a valid state yet.
    /// Equivalent to `!obj->has_state()` - inverse logic to make state packets smaller
    #[femtopb(bool, tag = 3)]
    pub missing_state: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== COVER ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesCoverResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(bool, tag = 5)]
    pub assumed_state: bool,
    #[femtopb(bool, tag = 6)]
    pub supports_position: bool,
    #[femtopb(bool, tag = 7)]
    pub supports_tilt: bool,
    #[femtopb(string, tag = 8)]
    pub device_class: &'a str,
    #[femtopb(bool, tag = 9)]
    pub disabled_by_default: bool,
    #[femtopb(string, tag = 10)]
    pub icon: &'a str,
    #[femtopb(enumeration, tag = 11)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(bool, tag = 12)]
    pub supports_stop: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct CoverStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    /// legacy: state has been removed in 1.13
    /// clients/servers must still send/accept it until the next protocol change
    #[femtopb(enumeration, tag = 2)]
    pub legacy_state: ::femtopb::enumeration::EnumValue<LegacyCoverState>,
    #[femtopb(float, tag = 3)]
    pub position: f32,
    #[femtopb(float, tag = 4)]
    pub tilt: f32,
    #[femtopb(enumeration, tag = 5)]
    pub current_operation: ::femtopb::enumeration::EnumValue<CoverOperation>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct CoverCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    /// legacy: command has been removed in 1.13
    /// clients/servers must still send/accept it until the next protocol change
    #[femtopb(bool, tag = 2)]
    pub has_legacy_command: bool,
    #[femtopb(enumeration, tag = 3)]
    pub legacy_command: ::femtopb::enumeration::EnumValue<LegacyCoverCommand>,
    #[femtopb(bool, tag = 4)]
    pub has_position: bool,
    #[femtopb(float, tag = 5)]
    pub position: f32,
    #[femtopb(bool, tag = 6)]
    pub has_tilt: bool,
    #[femtopb(float, tag = 7)]
    pub tilt: f32,
    #[femtopb(bool, tag = 8)]
    pub stop: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== FAN ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesFanResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(bool, tag = 5)]
    pub supports_oscillation: bool,
    #[femtopb(bool, tag = 6)]
    pub supports_speed: bool,
    #[femtopb(bool, tag = 7)]
    pub supports_direction: bool,
    #[femtopb(int32, tag = 8)]
    pub supported_speed_count: i32,
    #[femtopb(bool, tag = 9)]
    pub disabled_by_default: bool,
    #[femtopb(string, tag = 10)]
    pub icon: &'a str,
    #[femtopb(enumeration, tag = 11)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(string, repeated, tag = 12)]
    pub supported_preset_modes: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct FanStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bool, tag = 2)]
    pub state: bool,
    #[femtopb(bool, tag = 3)]
    pub oscillating: bool,
    #[deprecated]
    #[femtopb(enumeration, tag = 4)]
    pub speed: ::femtopb::enumeration::EnumValue<FanSpeed>,
    #[femtopb(enumeration, tag = 5)]
    pub direction: ::femtopb::enumeration::EnumValue<FanDirection>,
    #[femtopb(int32, tag = 6)]
    pub speed_level: i32,
    #[femtopb(string, tag = 7)]
    pub preset_mode: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct FanCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bool, tag = 2)]
    pub has_state: bool,
    #[femtopb(bool, tag = 3)]
    pub state: bool,
    #[deprecated]
    #[femtopb(bool, tag = 4)]
    pub has_speed: bool,
    #[deprecated]
    #[femtopb(enumeration, tag = 5)]
    pub speed: ::femtopb::enumeration::EnumValue<FanSpeed>,
    #[femtopb(bool, tag = 6)]
    pub has_oscillating: bool,
    #[femtopb(bool, tag = 7)]
    pub oscillating: bool,
    #[femtopb(bool, tag = 8)]
    pub has_direction: bool,
    #[femtopb(enumeration, tag = 9)]
    pub direction: ::femtopb::enumeration::EnumValue<FanDirection>,
    #[femtopb(bool, tag = 10)]
    pub has_speed_level: bool,
    #[femtopb(int32, tag = 11)]
    pub speed_level: i32,
    #[femtopb(bool, tag = 12)]
    pub has_preset_mode: bool,
    #[femtopb(string, tag = 13)]
    pub preset_mode: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesLightResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(enumeration, packed, tag = 12)]
    pub supported_color_modes: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<ColorMode>,
        ::femtopb::item_encoding::Enum<ColorMode>,
    >,
    /// next four supports_* are for legacy clients, newer clients should use color modes
    #[deprecated]
    #[femtopb(bool, tag = 5)]
    pub legacy_supports_brightness: bool,
    #[deprecated]
    #[femtopb(bool, tag = 6)]
    pub legacy_supports_rgb: bool,
    #[deprecated]
    #[femtopb(bool, tag = 7)]
    pub legacy_supports_white_value: bool,
    #[deprecated]
    #[femtopb(bool, tag = 8)]
    pub legacy_supports_color_temperature: bool,
    #[femtopb(float, tag = 9)]
    pub min_mireds: f32,
    #[femtopb(float, tag = 10)]
    pub max_mireds: f32,
    #[femtopb(string, repeated, tag = 11)]
    pub effects: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(bool, tag = 13)]
    pub disabled_by_default: bool,
    #[femtopb(string, tag = 14)]
    pub icon: &'a str,
    #[femtopb(enumeration, tag = 15)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct LightStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bool, tag = 2)]
    pub state: bool,
    #[femtopb(float, tag = 3)]
    pub brightness: f32,
    #[femtopb(enumeration, tag = 11)]
    pub color_mode: ::femtopb::enumeration::EnumValue<ColorMode>,
    #[femtopb(float, tag = 10)]
    pub color_brightness: f32,
    #[femtopb(float, tag = 4)]
    pub red: f32,
    #[femtopb(float, tag = 5)]
    pub green: f32,
    #[femtopb(float, tag = 6)]
    pub blue: f32,
    #[femtopb(float, tag = 7)]
    pub white: f32,
    #[femtopb(float, tag = 8)]
    pub color_temperature: f32,
    #[femtopb(float, tag = 12)]
    pub cold_white: f32,
    #[femtopb(float, tag = 13)]
    pub warm_white: f32,
    #[femtopb(string, tag = 9)]
    pub effect: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct LightCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bool, tag = 2)]
    pub has_state: bool,
    #[femtopb(bool, tag = 3)]
    pub state: bool,
    #[femtopb(bool, tag = 4)]
    pub has_brightness: bool,
    #[femtopb(float, tag = 5)]
    pub brightness: f32,
    #[femtopb(bool, tag = 22)]
    pub has_color_mode: bool,
    #[femtopb(enumeration, tag = 23)]
    pub color_mode: ::femtopb::enumeration::EnumValue<ColorMode>,
    #[femtopb(bool, tag = 20)]
    pub has_color_brightness: bool,
    #[femtopb(float, tag = 21)]
    pub color_brightness: f32,
    #[femtopb(bool, tag = 6)]
    pub has_rgb: bool,
    #[femtopb(float, tag = 7)]
    pub red: f32,
    #[femtopb(float, tag = 8)]
    pub green: f32,
    #[femtopb(float, tag = 9)]
    pub blue: f32,
    #[femtopb(bool, tag = 10)]
    pub has_white: bool,
    #[femtopb(float, tag = 11)]
    pub white: f32,
    #[femtopb(bool, tag = 12)]
    pub has_color_temperature: bool,
    #[femtopb(float, tag = 13)]
    pub color_temperature: f32,
    #[femtopb(bool, tag = 24)]
    pub has_cold_white: bool,
    #[femtopb(float, tag = 25)]
    pub cold_white: f32,
    #[femtopb(bool, tag = 26)]
    pub has_warm_white: bool,
    #[femtopb(float, tag = 27)]
    pub warm_white: f32,
    #[femtopb(bool, tag = 14)]
    pub has_transition_length: bool,
    #[femtopb(uint32, tag = 15)]
    pub transition_length: u32,
    #[femtopb(bool, tag = 16)]
    pub has_flash_length: bool,
    #[femtopb(uint32, tag = 17)]
    pub flash_length: u32,
    #[femtopb(bool, tag = 18)]
    pub has_effect: bool,
    #[femtopb(string, tag = 19)]
    pub effect: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesSensorResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(string, tag = 6)]
    pub unit_of_measurement: &'a str,
    #[femtopb(int32, tag = 7)]
    pub accuracy_decimals: i32,
    #[femtopb(bool, tag = 8)]
    pub force_update: bool,
    #[femtopb(string, tag = 9)]
    pub device_class: &'a str,
    #[femtopb(enumeration, tag = 10)]
    pub state_class: ::femtopb::enumeration::EnumValue<SensorStateClass>,
    /// Last reset type removed in 2021.9.0
    #[femtopb(enumeration, tag = 11)]
    pub legacy_last_reset_type: ::femtopb::enumeration::EnumValue<SensorLastResetType>,
    #[femtopb(bool, tag = 12)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 13)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct SensorStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(float, tag = 2)]
    pub state: f32,
    /// If the sensor does not have a valid state yet.
    /// Equivalent to `!obj->has_state()` - inverse logic to make state packets smaller
    #[femtopb(bool, tag = 3)]
    pub missing_state: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== SWITCH ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesSwitchResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub assumed_state: bool,
    #[femtopb(bool, tag = 7)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 8)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(string, tag = 9)]
    pub device_class: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct SwitchStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bool, tag = 2)]
    pub state: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct SwitchCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bool, tag = 2)]
    pub state: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== TEXT SENSOR ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesTextSensorResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 7)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(string, tag = 8)]
    pub device_class: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TextSensorStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(string, tag = 2)]
    pub state: &'a str,
    /// If the text sensor does not have a valid state yet.
    /// Equivalent to `!obj->has_state()` - inverse logic to make state packets smaller
    #[femtopb(bool, tag = 3)]
    pub missing_state: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct SubscribeLogsRequest<'a> {
    #[femtopb(enumeration, tag = 1)]
    pub level: ::femtopb::enumeration::EnumValue<LogLevel>,
    #[femtopb(bool, tag = 2)]
    pub dump_config: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct SubscribeLogsResponse<'a> {
    #[femtopb(enumeration, tag = 1)]
    pub level: ::femtopb::enumeration::EnumValue<LogLevel>,
    #[femtopb(bytes, tag = 3)]
    pub message: &'a [u8],
    #[femtopb(bool, tag = 4)]
    pub send_failed: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== NOISE ENCRYPTION ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct NoiseEncryptionSetKeyRequest<'a> {
    #[femtopb(bytes, tag = 1)]
    pub key: &'a [u8],
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct NoiseEncryptionSetKeyResponse<'a> {
    #[femtopb(bool, tag = 1)]
    pub success: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== HOMEASSISTANT.SERVICE ====================
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct SubscribeHomeassistantServicesRequest<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct HomeassistantServiceMap<'a> {
    #[femtopb(string, tag = 1)]
    pub key: &'a str,
    #[femtopb(string, tag = 2)]
    pub value: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct HomeassistantServiceResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub service: &'a str,
    #[femtopb(message, repeated, tag = 2)]
    pub data: ::femtopb::repeated::Repeated<
        'a,
        HomeassistantServiceMap<'a>,
        ::femtopb::item_encoding::Message<'a, HomeassistantServiceMap<'a>>,
    >,
    #[femtopb(message, repeated, tag = 3)]
    pub data_template: ::femtopb::repeated::Repeated<
        'a,
        HomeassistantServiceMap<'a>,
        ::femtopb::item_encoding::Message<'a, HomeassistantServiceMap<'a>>,
    >,
    #[femtopb(message, repeated, tag = 4)]
    pub variables: ::femtopb::repeated::Repeated<
        'a,
        HomeassistantServiceMap<'a>,
        ::femtopb::item_encoding::Message<'a, HomeassistantServiceMap<'a>>,
    >,
    #[femtopb(bool, tag = 5)]
    pub is_event: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== IMPORT HOME ASSISTANT STATES ====================
/// 1. Client sends SubscribeHomeAssistantStatesRequest
/// 2. Server responds with zero or more SubscribeHomeAssistantStateResponse (async)
/// 3. Client sends HomeAssistantStateResponse for state changes.
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct SubscribeHomeAssistantStatesRequest<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct SubscribeHomeAssistantStateResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub entity_id: &'a str,
    #[femtopb(string, tag = 2)]
    pub attribute: &'a str,
    #[femtopb(bool, tag = 3)]
    pub once: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct HomeAssistantStateResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub entity_id: &'a str,
    #[femtopb(string, tag = 2)]
    pub state: &'a str,
    #[femtopb(string, tag = 3)]
    pub attribute: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== IMPORT TIME ====================
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct GetTimeRequest<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct GetTimeResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub epoch_seconds: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesServicesArgument<'a> {
    #[femtopb(string, tag = 1)]
    pub name: &'a str,
    #[femtopb(enumeration, tag = 2)]
    pub r#type: ::femtopb::enumeration::EnumValue<ServiceArgType>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesServicesResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub name: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(message, repeated, tag = 3)]
    pub args: ::femtopb::repeated::Repeated<
        'a,
        ListEntitiesServicesArgument<'a>,
        ::femtopb::item_encoding::Message<'a, ListEntitiesServicesArgument<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ExecuteServiceArgument<'a> {
    #[femtopb(bool, tag = 1)]
    pub bool: bool,
    #[femtopb(int32, tag = 2)]
    pub legacy_int: i32,
    #[femtopb(float, tag = 3)]
    pub float: f32,
    #[femtopb(string, tag = 4)]
    pub string: &'a str,
    /// ESPHome 1.14 (api v1.3) make int a signed value
    #[femtopb(sint32, tag = 5)]
    pub int: i32,
    #[femtopb(bool, repeated, tag = 6)]
    pub bool_array: ::femtopb::repeated::Repeated<
        'a,
        bool,
        ::femtopb::item_encoding::Bool,
    >,
    #[femtopb(sint32, repeated, tag = 7)]
    pub int_array: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::SInt32,
    >,
    #[femtopb(float, repeated, tag = 8)]
    pub float_array: ::femtopb::repeated::Repeated<
        'a,
        f32,
        ::femtopb::item_encoding::Float,
    >,
    #[femtopb(string, repeated, tag = 9)]
    pub string_array: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ExecuteServiceRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(message, repeated, tag = 2)]
    pub args: ::femtopb::repeated::Repeated<
        'a,
        ExecuteServiceArgument<'a>,
        ::femtopb::item_encoding::Message<'a, ExecuteServiceArgument<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== CAMERA ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesCameraResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(bool, tag = 5)]
    pub disabled_by_default: bool,
    #[femtopb(string, tag = 6)]
    pub icon: &'a str,
    #[femtopb(enumeration, tag = 7)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct CameraImageResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bytes, tag = 2)]
    pub data: &'a [u8],
    #[femtopb(bool, tag = 3)]
    pub done: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct CameraImageRequest<'a> {
    #[femtopb(bool, tag = 1)]
    pub single: bool,
    #[femtopb(bool, tag = 2)]
    pub stream: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesClimateResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(bool, tag = 5)]
    pub supports_current_temperature: bool,
    #[femtopb(bool, tag = 6)]
    pub supports_two_point_target_temperature: bool,
    #[femtopb(enumeration, packed, tag = 7)]
    pub supported_modes: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<ClimateMode>,
        ::femtopb::item_encoding::Enum<ClimateMode>,
    >,
    #[femtopb(float, tag = 8)]
    pub visual_min_temperature: f32,
    #[femtopb(float, tag = 9)]
    pub visual_max_temperature: f32,
    #[femtopb(float, tag = 10)]
    pub visual_target_temperature_step: f32,
    /// for older peer versions - in new system this
    /// is if CLIMATE_PRESET_AWAY exists is supported_presets
    #[femtopb(bool, tag = 11)]
    pub legacy_supports_away: bool,
    #[femtopb(bool, tag = 12)]
    pub supports_action: bool,
    #[femtopb(enumeration, packed, tag = 13)]
    pub supported_fan_modes: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<ClimateFanMode>,
        ::femtopb::item_encoding::Enum<ClimateFanMode>,
    >,
    #[femtopb(enumeration, packed, tag = 14)]
    pub supported_swing_modes: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<ClimateSwingMode>,
        ::femtopb::item_encoding::Enum<ClimateSwingMode>,
    >,
    #[femtopb(string, repeated, tag = 15)]
    pub supported_custom_fan_modes: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(enumeration, packed, tag = 16)]
    pub supported_presets: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<ClimatePreset>,
        ::femtopb::item_encoding::Enum<ClimatePreset>,
    >,
    #[femtopb(string, repeated, tag = 17)]
    pub supported_custom_presets: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(bool, tag = 18)]
    pub disabled_by_default: bool,
    #[femtopb(string, tag = 19)]
    pub icon: &'a str,
    #[femtopb(enumeration, tag = 20)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(float, tag = 21)]
    pub visual_current_temperature_step: f32,
    #[femtopb(bool, tag = 22)]
    pub supports_current_humidity: bool,
    #[femtopb(bool, tag = 23)]
    pub supports_target_humidity: bool,
    #[femtopb(float, tag = 24)]
    pub visual_min_humidity: f32,
    #[femtopb(float, tag = 25)]
    pub visual_max_humidity: f32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ClimateStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(enumeration, tag = 2)]
    pub mode: ::femtopb::enumeration::EnumValue<ClimateMode>,
    #[femtopb(float, tag = 3)]
    pub current_temperature: f32,
    #[femtopb(float, tag = 4)]
    pub target_temperature: f32,
    #[femtopb(float, tag = 5)]
    pub target_temperature_low: f32,
    #[femtopb(float, tag = 6)]
    pub target_temperature_high: f32,
    /// For older peers, equal to preset == CLIMATE_PRESET_AWAY
    #[femtopb(bool, tag = 7)]
    pub unused_legacy_away: bool,
    #[femtopb(enumeration, tag = 8)]
    pub action: ::femtopb::enumeration::EnumValue<ClimateAction>,
    #[femtopb(enumeration, tag = 9)]
    pub fan_mode: ::femtopb::enumeration::EnumValue<ClimateFanMode>,
    #[femtopb(enumeration, tag = 10)]
    pub swing_mode: ::femtopb::enumeration::EnumValue<ClimateSwingMode>,
    #[femtopb(string, tag = 11)]
    pub custom_fan_mode: &'a str,
    #[femtopb(enumeration, tag = 12)]
    pub preset: ::femtopb::enumeration::EnumValue<ClimatePreset>,
    #[femtopb(string, tag = 13)]
    pub custom_preset: &'a str,
    #[femtopb(float, tag = 14)]
    pub current_humidity: f32,
    #[femtopb(float, tag = 15)]
    pub target_humidity: f32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ClimateCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bool, tag = 2)]
    pub has_mode: bool,
    #[femtopb(enumeration, tag = 3)]
    pub mode: ::femtopb::enumeration::EnumValue<ClimateMode>,
    #[femtopb(bool, tag = 4)]
    pub has_target_temperature: bool,
    #[femtopb(float, tag = 5)]
    pub target_temperature: f32,
    #[femtopb(bool, tag = 6)]
    pub has_target_temperature_low: bool,
    #[femtopb(float, tag = 7)]
    pub target_temperature_low: f32,
    #[femtopb(bool, tag = 8)]
    pub has_target_temperature_high: bool,
    #[femtopb(float, tag = 9)]
    pub target_temperature_high: f32,
    /// legacy, for older peers, newer ones should use CLIMATE_PRESET_AWAY in preset
    #[femtopb(bool, tag = 10)]
    pub unused_has_legacy_away: bool,
    #[femtopb(bool, tag = 11)]
    pub unused_legacy_away: bool,
    #[femtopb(bool, tag = 12)]
    pub has_fan_mode: bool,
    #[femtopb(enumeration, tag = 13)]
    pub fan_mode: ::femtopb::enumeration::EnumValue<ClimateFanMode>,
    #[femtopb(bool, tag = 14)]
    pub has_swing_mode: bool,
    #[femtopb(enumeration, tag = 15)]
    pub swing_mode: ::femtopb::enumeration::EnumValue<ClimateSwingMode>,
    #[femtopb(bool, tag = 16)]
    pub has_custom_fan_mode: bool,
    #[femtopb(string, tag = 17)]
    pub custom_fan_mode: &'a str,
    #[femtopb(bool, tag = 18)]
    pub has_preset: bool,
    #[femtopb(enumeration, tag = 19)]
    pub preset: ::femtopb::enumeration::EnumValue<ClimatePreset>,
    #[femtopb(bool, tag = 20)]
    pub has_custom_preset: bool,
    #[femtopb(string, tag = 21)]
    pub custom_preset: &'a str,
    #[femtopb(bool, tag = 22)]
    pub has_target_humidity: bool,
    #[femtopb(float, tag = 23)]
    pub target_humidity: f32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesNumberResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(float, tag = 6)]
    pub min_value: f32,
    #[femtopb(float, tag = 7)]
    pub max_value: f32,
    #[femtopb(float, tag = 8)]
    pub step: f32,
    #[femtopb(bool, tag = 9)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 10)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(string, tag = 11)]
    pub unit_of_measurement: &'a str,
    #[femtopb(enumeration, tag = 12)]
    pub mode: ::femtopb::enumeration::EnumValue<NumberMode>,
    #[femtopb(string, tag = 13)]
    pub device_class: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct NumberStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(float, tag = 2)]
    pub state: f32,
    /// If the number does not have a valid state yet.
    /// Equivalent to `!obj->has_state()` - inverse logic to make state packets smaller
    #[femtopb(bool, tag = 3)]
    pub missing_state: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct NumberCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(float, tag = 2)]
    pub state: f32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== SELECT ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesSelectResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(string, repeated, tag = 6)]
    pub options: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(bool, tag = 7)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 8)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct SelectStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(string, tag = 2)]
    pub state: &'a str,
    /// If the select does not have a valid state yet.
    /// Equivalent to `!obj->has_state()` - inverse logic to make state packets smaller
    #[femtopb(bool, tag = 3)]
    pub missing_state: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct SelectCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(string, tag = 2)]
    pub state: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== SIREN ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesSirenResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub disabled_by_default: bool,
    #[femtopb(string, repeated, tag = 7)]
    pub tones: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(bool, tag = 8)]
    pub supports_duration: bool,
    #[femtopb(bool, tag = 9)]
    pub supports_volume: bool,
    #[femtopb(enumeration, tag = 10)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct SirenStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bool, tag = 2)]
    pub state: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct SirenCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bool, tag = 2)]
    pub has_state: bool,
    #[femtopb(bool, tag = 3)]
    pub state: bool,
    #[femtopb(bool, tag = 4)]
    pub has_tone: bool,
    #[femtopb(string, tag = 5)]
    pub tone: &'a str,
    #[femtopb(bool, tag = 6)]
    pub has_duration: bool,
    #[femtopb(uint32, tag = 7)]
    pub duration: u32,
    #[femtopb(bool, tag = 8)]
    pub has_volume: bool,
    #[femtopb(float, tag = 9)]
    pub volume: f32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesLockResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 7)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(bool, tag = 8)]
    pub assumed_state: bool,
    #[femtopb(bool, tag = 9)]
    pub supports_open: bool,
    #[femtopb(bool, tag = 10)]
    pub requires_code: bool,
    /// Not yet implemented:
    #[femtopb(string, tag = 11)]
    pub code_format: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct LockStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(enumeration, tag = 2)]
    pub state: ::femtopb::enumeration::EnumValue<LockState>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct LockCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(enumeration, tag = 2)]
    pub command: ::femtopb::enumeration::EnumValue<LockCommand>,
    /// Not yet implemented:
    #[femtopb(bool, tag = 3)]
    pub has_code: bool,
    #[femtopb(string, tag = 4)]
    pub code: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== BUTTON ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesButtonResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 7)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(string, tag = 8)]
    pub device_class: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct ButtonCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct MediaPlayerSupportedFormat<'a> {
    #[femtopb(string, tag = 1)]
    pub format: &'a str,
    #[femtopb(uint32, tag = 2)]
    pub sample_rate: u32,
    #[femtopb(uint32, tag = 3)]
    pub num_channels: u32,
    #[femtopb(enumeration, tag = 4)]
    pub purpose: ::femtopb::enumeration::EnumValue<MediaPlayerFormatPurpose>,
    #[femtopb(uint32, tag = 5)]
    pub sample_bytes: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesMediaPlayerResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 7)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(bool, tag = 8)]
    pub supports_pause: bool,
    #[femtopb(message, repeated, tag = 9)]
    pub supported_formats: ::femtopb::repeated::Repeated<
        'a,
        MediaPlayerSupportedFormat<'a>,
        ::femtopb::item_encoding::Message<'a, MediaPlayerSupportedFormat<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct MediaPlayerStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(enumeration, tag = 2)]
    pub state: ::femtopb::enumeration::EnumValue<MediaPlayerState>,
    #[femtopb(float, tag = 3)]
    pub volume: f32,
    #[femtopb(bool, tag = 4)]
    pub muted: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct MediaPlayerCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bool, tag = 2)]
    pub has_command: bool,
    #[femtopb(enumeration, tag = 3)]
    pub command: ::femtopb::enumeration::EnumValue<MediaPlayerCommand>,
    #[femtopb(bool, tag = 4)]
    pub has_volume: bool,
    #[femtopb(float, tag = 5)]
    pub volume: f32,
    #[femtopb(bool, tag = 6)]
    pub has_media_url: bool,
    #[femtopb(string, tag = 7)]
    pub media_url: &'a str,
    #[femtopb(bool, tag = 8)]
    pub has_announcement: bool,
    #[femtopb(bool, tag = 9)]
    pub announcement: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== BLUETOOTH ====================
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct SubscribeBluetoothLeAdvertisementsRequest<'a> {
    #[femtopb(uint32, tag = 1)]
    pub flags: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BluetoothServiceData<'a> {
    #[femtopb(string, tag = 1)]
    pub uuid: &'a str,
    /// Removed in api version 1.7
    #[deprecated]
    #[femtopb(uint32, repeated, tag = 2)]
    pub legacy_data: ::femtopb::repeated::Repeated<
        'a,
        u32,
        ::femtopb::item_encoding::UInt32,
    >,
    /// Added in api version 1.7
    #[femtopb(bytes, tag = 3)]
    pub data: &'a [u8],
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BluetoothLeAdvertisementResponse<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(bytes, tag = 2)]
    pub name: &'a [u8],
    #[femtopb(sint32, tag = 3)]
    pub rssi: i32,
    #[femtopb(string, repeated, tag = 4)]
    pub service_uuids: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(message, repeated, tag = 5)]
    pub service_data: ::femtopb::repeated::Repeated<
        'a,
        BluetoothServiceData<'a>,
        ::femtopb::item_encoding::Message<'a, BluetoothServiceData<'a>>,
    >,
    #[femtopb(message, repeated, tag = 6)]
    pub manufacturer_data: ::femtopb::repeated::Repeated<
        'a,
        BluetoothServiceData<'a>,
        ::femtopb::item_encoding::Message<'a, BluetoothServiceData<'a>>,
    >,
    #[femtopb(uint32, tag = 7)]
    pub address_type: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BluetoothLeRawAdvertisement<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(sint32, tag = 2)]
    pub rssi: i32,
    #[femtopb(uint32, tag = 3)]
    pub address_type: u32,
    #[femtopb(bytes, tag = 4)]
    pub data: &'a [u8],
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BluetoothLeRawAdvertisementsResponse<'a> {
    #[femtopb(message, repeated, tag = 1)]
    pub advertisements: ::femtopb::repeated::Repeated<
        'a,
        BluetoothLeRawAdvertisement<'a>,
        ::femtopb::item_encoding::Message<'a, BluetoothLeRawAdvertisement<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothDeviceRequest<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(enumeration, tag = 2)]
    pub request_type: ::femtopb::enumeration::EnumValue<BluetoothDeviceRequestType>,
    #[femtopb(bool, tag = 3)]
    pub has_address_type: bool,
    #[femtopb(uint32, tag = 4)]
    pub address_type: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothDeviceConnectionResponse<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(bool, tag = 2)]
    pub connected: bool,
    #[femtopb(uint32, tag = 3)]
    pub mtu: u32,
    #[femtopb(int32, tag = 4)]
    pub error: i32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattGetServicesRequest<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattDescriptor<'a> {
    #[femtopb(uint64, packed, tag = 1)]
    pub uuid: ::femtopb::packed::Packed<'a, u64, ::femtopb::item_encoding::UInt64>,
    #[femtopb(uint32, tag = 2)]
    pub handle: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattCharacteristic<'a> {
    #[femtopb(uint64, packed, tag = 1)]
    pub uuid: ::femtopb::packed::Packed<'a, u64, ::femtopb::item_encoding::UInt64>,
    #[femtopb(uint32, tag = 2)]
    pub handle: u32,
    #[femtopb(uint32, tag = 3)]
    pub properties: u32,
    #[femtopb(message, repeated, tag = 4)]
    pub descriptors: ::femtopb::repeated::Repeated<
        'a,
        BluetoothGattDescriptor<'a>,
        ::femtopb::item_encoding::Message<'a, BluetoothGattDescriptor<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattService<'a> {
    #[femtopb(uint64, packed, tag = 1)]
    pub uuid: ::femtopb::packed::Packed<'a, u64, ::femtopb::item_encoding::UInt64>,
    #[femtopb(uint32, tag = 2)]
    pub handle: u32,
    #[femtopb(message, repeated, tag = 3)]
    pub characteristics: ::femtopb::repeated::Repeated<
        'a,
        BluetoothGattCharacteristic<'a>,
        ::femtopb::item_encoding::Message<'a, BluetoothGattCharacteristic<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattGetServicesResponse<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(message, repeated, tag = 2)]
    pub services: ::femtopb::repeated::Repeated<
        'a,
        BluetoothGattService<'a>,
        ::femtopb::item_encoding::Message<'a, BluetoothGattService<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattGetServicesDoneResponse<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattReadRequest<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(uint32, tag = 2)]
    pub handle: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattReadResponse<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(uint32, tag = 2)]
    pub handle: u32,
    #[femtopb(bytes, tag = 3)]
    pub data: &'a [u8],
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattWriteRequest<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(uint32, tag = 2)]
    pub handle: u32,
    #[femtopb(bool, tag = 3)]
    pub response: bool,
    #[femtopb(bytes, tag = 4)]
    pub data: &'a [u8],
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattReadDescriptorRequest<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(uint32, tag = 2)]
    pub handle: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattWriteDescriptorRequest<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(uint32, tag = 2)]
    pub handle: u32,
    #[femtopb(bytes, tag = 3)]
    pub data: &'a [u8],
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattNotifyRequest<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(uint32, tag = 2)]
    pub handle: u32,
    #[femtopb(bool, tag = 3)]
    pub enable: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattNotifyDataResponse<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(uint32, tag = 2)]
    pub handle: u32,
    #[femtopb(bytes, tag = 3)]
    pub data: &'a [u8],
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct SubscribeBluetoothConnectionsFreeRequest<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BluetoothConnectionsFreeResponse<'a> {
    #[femtopb(uint32, tag = 1)]
    pub free: u32,
    #[femtopb(uint32, tag = 2)]
    pub limit: u32,
    #[femtopb(uint64, packed, tag = 3)]
    pub allocated: ::femtopb::packed::Packed<'a, u64, ::femtopb::item_encoding::UInt64>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattErrorResponse<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(uint32, tag = 2)]
    pub handle: u32,
    #[femtopb(int32, tag = 3)]
    pub error: i32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattWriteResponse<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(uint32, tag = 2)]
    pub handle: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothGattNotifyResponse<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(uint32, tag = 2)]
    pub handle: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothDevicePairingResponse<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(bool, tag = 2)]
    pub paired: bool,
    #[femtopb(int32, tag = 3)]
    pub error: i32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothDeviceUnpairingResponse<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(bool, tag = 2)]
    pub success: bool,
    #[femtopb(int32, tag = 3)]
    pub error: i32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct UnsubscribeBluetoothLeAdvertisementsRequest<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothDeviceClearCacheResponse<'a> {
    #[femtopb(uint64, tag = 1)]
    pub address: u64,
    #[femtopb(bool, tag = 2)]
    pub success: bool,
    #[femtopb(int32, tag = 3)]
    pub error: i32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothScannerStateResponse<'a> {
    #[femtopb(enumeration, tag = 1)]
    pub state: ::femtopb::enumeration::EnumValue<BluetoothScannerState>,
    #[femtopb(enumeration, tag = 2)]
    pub mode: ::femtopb::enumeration::EnumValue<BluetoothScannerMode>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothScannerSetModeRequest<'a> {
    #[femtopb(enumeration, tag = 1)]
    pub mode: ::femtopb::enumeration::EnumValue<BluetoothScannerMode>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct SubscribeVoiceAssistantRequest<'a> {
    #[femtopb(bool, tag = 1)]
    pub subscribe: bool,
    #[femtopb(uint32, tag = 2)]
    pub flags: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct VoiceAssistantAudioSettings<'a> {
    #[femtopb(uint32, tag = 1)]
    pub noise_suppression_level: u32,
    #[femtopb(uint32, tag = 2)]
    pub auto_gain: u32,
    #[femtopb(float, tag = 3)]
    pub volume_multiplier: f32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct VoiceAssistantRequest<'a> {
    #[femtopb(bool, tag = 1)]
    pub start: bool,
    #[femtopb(string, tag = 2)]
    pub conversation_id: &'a str,
    #[femtopb(uint32, tag = 3)]
    pub flags: u32,
    #[femtopb(message, optional, tag = 4)]
    pub audio_settings: ::core::option::Option<VoiceAssistantAudioSettings<'a>>,
    #[femtopb(string, tag = 5)]
    pub wake_word_phrase: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct VoiceAssistantResponse<'a> {
    #[femtopb(uint32, tag = 1)]
    pub port: u32,
    #[femtopb(bool, tag = 2)]
    pub error: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct VoiceAssistantEventData<'a> {
    #[femtopb(string, tag = 1)]
    pub name: &'a str,
    #[femtopb(string, tag = 2)]
    pub value: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct VoiceAssistantEventResponse<'a> {
    #[femtopb(enumeration, tag = 1)]
    pub event_type: ::femtopb::enumeration::EnumValue<VoiceAssistantEvent>,
    #[femtopb(message, repeated, tag = 2)]
    pub data: ::femtopb::repeated::Repeated<
        'a,
        VoiceAssistantEventData<'a>,
        ::femtopb::item_encoding::Message<'a, VoiceAssistantEventData<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct VoiceAssistantAudio<'a> {
    #[femtopb(bytes, tag = 1)]
    pub data: &'a [u8],
    #[femtopb(bool, tag = 2)]
    pub end: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct VoiceAssistantTimerEventResponse<'a> {
    #[femtopb(enumeration, tag = 1)]
    pub event_type: ::femtopb::enumeration::EnumValue<VoiceAssistantTimerEvent>,
    #[femtopb(string, tag = 2)]
    pub timer_id: &'a str,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(uint32, tag = 4)]
    pub total_seconds: u32,
    #[femtopb(uint32, tag = 5)]
    pub seconds_left: u32,
    #[femtopb(bool, tag = 6)]
    pub is_active: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct VoiceAssistantAnnounceRequest<'a> {
    #[femtopb(string, tag = 1)]
    pub media_id: &'a str,
    #[femtopb(string, tag = 2)]
    pub text: &'a str,
    #[femtopb(string, tag = 3)]
    pub preannounce_media_id: &'a str,
    #[femtopb(bool, tag = 4)]
    pub start_conversation: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct VoiceAssistantAnnounceFinished<'a> {
    #[femtopb(bool, tag = 1)]
    pub success: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct VoiceAssistantWakeWord<'a> {
    #[femtopb(string, tag = 1)]
    pub id: &'a str,
    #[femtopb(string, tag = 2)]
    pub wake_word: &'a str,
    #[femtopb(string, repeated, tag = 3)]
    pub trained_languages: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct VoiceAssistantConfigurationRequest<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct VoiceAssistantConfigurationResponse<'a> {
    #[femtopb(message, repeated, tag = 1)]
    pub available_wake_words: ::femtopb::repeated::Repeated<
        'a,
        VoiceAssistantWakeWord<'a>,
        ::femtopb::item_encoding::Message<'a, VoiceAssistantWakeWord<'a>>,
    >,
    #[femtopb(string, repeated, tag = 2)]
    pub active_wake_words: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(uint32, tag = 3)]
    pub max_active_wake_words: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct VoiceAssistantSetConfiguration<'a> {
    #[femtopb(string, repeated, tag = 1)]
    pub active_wake_words: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesAlarmControlPanelResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 7)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(uint32, tag = 8)]
    pub supported_features: u32,
    #[femtopb(bool, tag = 9)]
    pub requires_code: bool,
    #[femtopb(bool, tag = 10)]
    pub requires_code_to_arm: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct AlarmControlPanelStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(enumeration, tag = 2)]
    pub state: ::femtopb::enumeration::EnumValue<AlarmControlPanelState>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct AlarmControlPanelCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(enumeration, tag = 2)]
    pub command: ::femtopb::enumeration::EnumValue<AlarmControlPanelStateCommand>,
    #[femtopb(string, tag = 3)]
    pub code: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesTextResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 7)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(uint32, tag = 8)]
    pub min_length: u32,
    #[femtopb(uint32, tag = 9)]
    pub max_length: u32,
    #[femtopb(string, tag = 10)]
    pub pattern: &'a str,
    #[femtopb(enumeration, tag = 11)]
    pub mode: ::femtopb::enumeration::EnumValue<TextMode>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TextStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(string, tag = 2)]
    pub state: &'a str,
    /// If the Text does not have a valid state yet.
    /// Equivalent to `!obj->has_state()` - inverse logic to make state packets smaller
    #[femtopb(bool, tag = 3)]
    pub missing_state: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TextCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(string, tag = 2)]
    pub state: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== DATETIME DATE ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesDateResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 7)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct DateStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    /// If the date does not have a valid state yet.
    /// Equivalent to `!obj->has_state()` - inverse logic to make state packets smaller
    #[femtopb(bool, tag = 2)]
    pub missing_state: bool,
    #[femtopb(uint32, tag = 3)]
    pub year: u32,
    #[femtopb(uint32, tag = 4)]
    pub month: u32,
    #[femtopb(uint32, tag = 5)]
    pub day: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct DateCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(uint32, tag = 2)]
    pub year: u32,
    #[femtopb(uint32, tag = 3)]
    pub month: u32,
    #[femtopb(uint32, tag = 4)]
    pub day: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== DATETIME TIME ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesTimeResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 7)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct TimeStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    /// If the time does not have a valid state yet.
    /// Equivalent to `!obj->has_state()` - inverse logic to make state packets smaller
    #[femtopb(bool, tag = 2)]
    pub missing_state: bool,
    #[femtopb(uint32, tag = 3)]
    pub hour: u32,
    #[femtopb(uint32, tag = 4)]
    pub minute: u32,
    #[femtopb(uint32, tag = 5)]
    pub second: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct TimeCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(uint32, tag = 2)]
    pub hour: u32,
    #[femtopb(uint32, tag = 3)]
    pub minute: u32,
    #[femtopb(uint32, tag = 4)]
    pub second: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== EVENT ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesEventResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 7)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(string, tag = 8)]
    pub device_class: &'a str,
    #[femtopb(string, repeated, tag = 9)]
    pub event_types: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct EventResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(string, tag = 2)]
    pub event_type: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== VALVE ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesValveResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 7)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(string, tag = 8)]
    pub device_class: &'a str,
    #[femtopb(bool, tag = 9)]
    pub assumed_state: bool,
    #[femtopb(bool, tag = 10)]
    pub supports_position: bool,
    #[femtopb(bool, tag = 11)]
    pub supports_stop: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct ValveStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(float, tag = 2)]
    pub position: f32,
    #[femtopb(enumeration, tag = 3)]
    pub current_operation: ::femtopb::enumeration::EnumValue<ValveOperation>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct ValveCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bool, tag = 2)]
    pub has_position: bool,
    #[femtopb(float, tag = 3)]
    pub position: f32,
    #[femtopb(bool, tag = 4)]
    pub stop: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== DATETIME DATETIME ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesDateTimeResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 7)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct DateTimeStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    /// If the datetime does not have a valid state yet.
    /// Equivalent to `!obj->has_state()` - inverse logic to make state packets smaller
    #[femtopb(bool, tag = 2)]
    pub missing_state: bool,
    #[femtopb(fixed32, tag = 3)]
    pub epoch_seconds: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct DateTimeCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(fixed32, tag = 2)]
    pub epoch_seconds: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// ==================== UPDATE ====================
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ListEntitiesUpdateResponse<'a> {
    #[femtopb(string, tag = 1)]
    pub object_id: &'a str,
    #[femtopb(fixed32, tag = 2)]
    pub key: u32,
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
    #[femtopb(string, tag = 4)]
    pub unique_id: &'a str,
    #[femtopb(string, tag = 5)]
    pub icon: &'a str,
    #[femtopb(bool, tag = 6)]
    pub disabled_by_default: bool,
    #[femtopb(enumeration, tag = 7)]
    pub entity_category: ::femtopb::enumeration::EnumValue<EntityCategory>,
    #[femtopb(string, tag = 8)]
    pub device_class: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct UpdateStateResponse<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(bool, tag = 2)]
    pub missing_state: bool,
    #[femtopb(bool, tag = 3)]
    pub in_progress: bool,
    #[femtopb(bool, tag = 4)]
    pub has_progress: bool,
    #[femtopb(float, tag = 5)]
    pub progress: f32,
    #[femtopb(string, tag = 6)]
    pub current_version: &'a str,
    #[femtopb(string, tag = 7)]
    pub latest_version: &'a str,
    #[femtopb(string, tag = 8)]
    pub title: &'a str,
    #[femtopb(string, tag = 9)]
    pub release_summary: &'a str,
    #[femtopb(string, tag = 10)]
    pub release_url: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct UpdateCommandRequest<'a> {
    #[femtopb(fixed32, tag = 1)]
    pub key: u32,
    #[femtopb(enumeration, tag = 2)]
    pub command: ::femtopb::enumeration::EnumValue<UpdateCommand>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum EntityCategory {
    #[default]
    None = 0,
    Config = 1,
    Diagnostic = 2,
}
impl EntityCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "ENTITY_CATEGORY_NONE",
            Self::Config => "ENTITY_CATEGORY_CONFIG",
            Self::Diagnostic => "ENTITY_CATEGORY_DIAGNOSTIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENTITY_CATEGORY_NONE" => Some(Self::None),
            "ENTITY_CATEGORY_CONFIG" => Some(Self::Config),
            "ENTITY_CATEGORY_DIAGNOSTIC" => Some(Self::Diagnostic),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum LegacyCoverState {
    #[default]
    Open = 0,
    Closed = 1,
}
impl LegacyCoverState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Open => "LEGACY_COVER_STATE_OPEN",
            Self::Closed => "LEGACY_COVER_STATE_CLOSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEGACY_COVER_STATE_OPEN" => Some(Self::Open),
            "LEGACY_COVER_STATE_CLOSED" => Some(Self::Closed),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum CoverOperation {
    #[default]
    Idle = 0,
    IsOpening = 1,
    IsClosing = 2,
}
impl CoverOperation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Idle => "COVER_OPERATION_IDLE",
            Self::IsOpening => "COVER_OPERATION_IS_OPENING",
            Self::IsClosing => "COVER_OPERATION_IS_CLOSING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COVER_OPERATION_IDLE" => Some(Self::Idle),
            "COVER_OPERATION_IS_OPENING" => Some(Self::IsOpening),
            "COVER_OPERATION_IS_CLOSING" => Some(Self::IsClosing),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum LegacyCoverCommand {
    #[default]
    Open = 0,
    Close = 1,
    Stop = 2,
}
impl LegacyCoverCommand {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Open => "LEGACY_COVER_COMMAND_OPEN",
            Self::Close => "LEGACY_COVER_COMMAND_CLOSE",
            Self::Stop => "LEGACY_COVER_COMMAND_STOP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEGACY_COVER_COMMAND_OPEN" => Some(Self::Open),
            "LEGACY_COVER_COMMAND_CLOSE" => Some(Self::Close),
            "LEGACY_COVER_COMMAND_STOP" => Some(Self::Stop),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum FanSpeed {
    #[default]
    Low = 0,
    Medium = 1,
    High = 2,
}
impl FanSpeed {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Low => "FAN_SPEED_LOW",
            Self::Medium => "FAN_SPEED_MEDIUM",
            Self::High => "FAN_SPEED_HIGH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FAN_SPEED_LOW" => Some(Self::Low),
            "FAN_SPEED_MEDIUM" => Some(Self::Medium),
            "FAN_SPEED_HIGH" => Some(Self::High),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum FanDirection {
    #[default]
    Forward = 0,
    Reverse = 1,
}
impl FanDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Forward => "FAN_DIRECTION_FORWARD",
            Self::Reverse => "FAN_DIRECTION_REVERSE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FAN_DIRECTION_FORWARD" => Some(Self::Forward),
            "FAN_DIRECTION_REVERSE" => Some(Self::Reverse),
            _ => None,
        }
    }
}
/// ==================== LIGHT ====================
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum ColorMode {
    #[default]
    Unknown = 0,
    OnOff = 1,
    LegacyBrightness = 2,
    Brightness = 3,
    White = 7,
    ColorTemperature = 11,
    ColdWarmWhite = 19,
    Rgb = 35,
    RgbWhite = 39,
    RgbColorTemperature = 47,
    RgbColdWarmWhite = 51,
}
impl ColorMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "COLOR_MODE_UNKNOWN",
            Self::OnOff => "COLOR_MODE_ON_OFF",
            Self::LegacyBrightness => "COLOR_MODE_LEGACY_BRIGHTNESS",
            Self::Brightness => "COLOR_MODE_BRIGHTNESS",
            Self::White => "COLOR_MODE_WHITE",
            Self::ColorTemperature => "COLOR_MODE_COLOR_TEMPERATURE",
            Self::ColdWarmWhite => "COLOR_MODE_COLD_WARM_WHITE",
            Self::Rgb => "COLOR_MODE_RGB",
            Self::RgbWhite => "COLOR_MODE_RGB_WHITE",
            Self::RgbColorTemperature => "COLOR_MODE_RGB_COLOR_TEMPERATURE",
            Self::RgbColdWarmWhite => "COLOR_MODE_RGB_COLD_WARM_WHITE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COLOR_MODE_UNKNOWN" => Some(Self::Unknown),
            "COLOR_MODE_ON_OFF" => Some(Self::OnOff),
            "COLOR_MODE_LEGACY_BRIGHTNESS" => Some(Self::LegacyBrightness),
            "COLOR_MODE_BRIGHTNESS" => Some(Self::Brightness),
            "COLOR_MODE_WHITE" => Some(Self::White),
            "COLOR_MODE_COLOR_TEMPERATURE" => Some(Self::ColorTemperature),
            "COLOR_MODE_COLD_WARM_WHITE" => Some(Self::ColdWarmWhite),
            "COLOR_MODE_RGB" => Some(Self::Rgb),
            "COLOR_MODE_RGB_WHITE" => Some(Self::RgbWhite),
            "COLOR_MODE_RGB_COLOR_TEMPERATURE" => Some(Self::RgbColorTemperature),
            "COLOR_MODE_RGB_COLD_WARM_WHITE" => Some(Self::RgbColdWarmWhite),
            _ => None,
        }
    }
}
/// ==================== SENSOR ====================
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum SensorStateClass {
    #[default]
    StateClassNone = 0,
    StateClassMeasurement = 1,
    StateClassTotalIncreasing = 2,
    StateClassTotal = 3,
}
impl SensorStateClass {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::StateClassNone => "STATE_CLASS_NONE",
            Self::StateClassMeasurement => "STATE_CLASS_MEASUREMENT",
            Self::StateClassTotalIncreasing => "STATE_CLASS_TOTAL_INCREASING",
            Self::StateClassTotal => "STATE_CLASS_TOTAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_CLASS_NONE" => Some(Self::StateClassNone),
            "STATE_CLASS_MEASUREMENT" => Some(Self::StateClassMeasurement),
            "STATE_CLASS_TOTAL_INCREASING" => Some(Self::StateClassTotalIncreasing),
            "STATE_CLASS_TOTAL" => Some(Self::StateClassTotal),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum SensorLastResetType {
    #[default]
    LastResetNone = 0,
    LastResetNever = 1,
    LastResetAuto = 2,
}
impl SensorLastResetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::LastResetNone => "LAST_RESET_NONE",
            Self::LastResetNever => "LAST_RESET_NEVER",
            Self::LastResetAuto => "LAST_RESET_AUTO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LAST_RESET_NONE" => Some(Self::LastResetNone),
            "LAST_RESET_NEVER" => Some(Self::LastResetNever),
            "LAST_RESET_AUTO" => Some(Self::LastResetAuto),
            _ => None,
        }
    }
}
/// ==================== SUBSCRIBE LOGS ====================
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum LogLevel {
    #[default]
    None = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Config = 4,
    Debug = 5,
    Verbose = 6,
    VeryVerbose = 7,
}
impl LogLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "LOG_LEVEL_NONE",
            Self::Error => "LOG_LEVEL_ERROR",
            Self::Warn => "LOG_LEVEL_WARN",
            Self::Info => "LOG_LEVEL_INFO",
            Self::Config => "LOG_LEVEL_CONFIG",
            Self::Debug => "LOG_LEVEL_DEBUG",
            Self::Verbose => "LOG_LEVEL_VERBOSE",
            Self::VeryVerbose => "LOG_LEVEL_VERY_VERBOSE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOG_LEVEL_NONE" => Some(Self::None),
            "LOG_LEVEL_ERROR" => Some(Self::Error),
            "LOG_LEVEL_WARN" => Some(Self::Warn),
            "LOG_LEVEL_INFO" => Some(Self::Info),
            "LOG_LEVEL_CONFIG" => Some(Self::Config),
            "LOG_LEVEL_DEBUG" => Some(Self::Debug),
            "LOG_LEVEL_VERBOSE" => Some(Self::Verbose),
            "LOG_LEVEL_VERY_VERBOSE" => Some(Self::VeryVerbose),
            _ => None,
        }
    }
}
/// ==================== USER-DEFINES SERVICES ====================
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum ServiceArgType {
    #[default]
    Bool = 0,
    Int = 1,
    Float = 2,
    String = 3,
    BoolArray = 4,
    IntArray = 5,
    FloatArray = 6,
    StringArray = 7,
}
impl ServiceArgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Bool => "SERVICE_ARG_TYPE_BOOL",
            Self::Int => "SERVICE_ARG_TYPE_INT",
            Self::Float => "SERVICE_ARG_TYPE_FLOAT",
            Self::String => "SERVICE_ARG_TYPE_STRING",
            Self::BoolArray => "SERVICE_ARG_TYPE_BOOL_ARRAY",
            Self::IntArray => "SERVICE_ARG_TYPE_INT_ARRAY",
            Self::FloatArray => "SERVICE_ARG_TYPE_FLOAT_ARRAY",
            Self::StringArray => "SERVICE_ARG_TYPE_STRING_ARRAY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SERVICE_ARG_TYPE_BOOL" => Some(Self::Bool),
            "SERVICE_ARG_TYPE_INT" => Some(Self::Int),
            "SERVICE_ARG_TYPE_FLOAT" => Some(Self::Float),
            "SERVICE_ARG_TYPE_STRING" => Some(Self::String),
            "SERVICE_ARG_TYPE_BOOL_ARRAY" => Some(Self::BoolArray),
            "SERVICE_ARG_TYPE_INT_ARRAY" => Some(Self::IntArray),
            "SERVICE_ARG_TYPE_FLOAT_ARRAY" => Some(Self::FloatArray),
            "SERVICE_ARG_TYPE_STRING_ARRAY" => Some(Self::StringArray),
            _ => None,
        }
    }
}
/// ==================== CLIMATE ====================
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum ClimateMode {
    #[default]
    Off = 0,
    HeatCool = 1,
    Cool = 2,
    Heat = 3,
    FanOnly = 4,
    Dry = 5,
    Auto = 6,
}
impl ClimateMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Off => "CLIMATE_MODE_OFF",
            Self::HeatCool => "CLIMATE_MODE_HEAT_COOL",
            Self::Cool => "CLIMATE_MODE_COOL",
            Self::Heat => "CLIMATE_MODE_HEAT",
            Self::FanOnly => "CLIMATE_MODE_FAN_ONLY",
            Self::Dry => "CLIMATE_MODE_DRY",
            Self::Auto => "CLIMATE_MODE_AUTO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLIMATE_MODE_OFF" => Some(Self::Off),
            "CLIMATE_MODE_HEAT_COOL" => Some(Self::HeatCool),
            "CLIMATE_MODE_COOL" => Some(Self::Cool),
            "CLIMATE_MODE_HEAT" => Some(Self::Heat),
            "CLIMATE_MODE_FAN_ONLY" => Some(Self::FanOnly),
            "CLIMATE_MODE_DRY" => Some(Self::Dry),
            "CLIMATE_MODE_AUTO" => Some(Self::Auto),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum ClimateFanMode {
    #[default]
    ClimateFanOn = 0,
    ClimateFanOff = 1,
    ClimateFanAuto = 2,
    ClimateFanLow = 3,
    ClimateFanMedium = 4,
    ClimateFanHigh = 5,
    ClimateFanMiddle = 6,
    ClimateFanFocus = 7,
    ClimateFanDiffuse = 8,
    ClimateFanQuiet = 9,
}
impl ClimateFanMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::ClimateFanOn => "CLIMATE_FAN_ON",
            Self::ClimateFanOff => "CLIMATE_FAN_OFF",
            Self::ClimateFanAuto => "CLIMATE_FAN_AUTO",
            Self::ClimateFanLow => "CLIMATE_FAN_LOW",
            Self::ClimateFanMedium => "CLIMATE_FAN_MEDIUM",
            Self::ClimateFanHigh => "CLIMATE_FAN_HIGH",
            Self::ClimateFanMiddle => "CLIMATE_FAN_MIDDLE",
            Self::ClimateFanFocus => "CLIMATE_FAN_FOCUS",
            Self::ClimateFanDiffuse => "CLIMATE_FAN_DIFFUSE",
            Self::ClimateFanQuiet => "CLIMATE_FAN_QUIET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLIMATE_FAN_ON" => Some(Self::ClimateFanOn),
            "CLIMATE_FAN_OFF" => Some(Self::ClimateFanOff),
            "CLIMATE_FAN_AUTO" => Some(Self::ClimateFanAuto),
            "CLIMATE_FAN_LOW" => Some(Self::ClimateFanLow),
            "CLIMATE_FAN_MEDIUM" => Some(Self::ClimateFanMedium),
            "CLIMATE_FAN_HIGH" => Some(Self::ClimateFanHigh),
            "CLIMATE_FAN_MIDDLE" => Some(Self::ClimateFanMiddle),
            "CLIMATE_FAN_FOCUS" => Some(Self::ClimateFanFocus),
            "CLIMATE_FAN_DIFFUSE" => Some(Self::ClimateFanDiffuse),
            "CLIMATE_FAN_QUIET" => Some(Self::ClimateFanQuiet),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum ClimateSwingMode {
    #[default]
    ClimateSwingOff = 0,
    ClimateSwingBoth = 1,
    ClimateSwingVertical = 2,
    ClimateSwingHorizontal = 3,
}
impl ClimateSwingMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::ClimateSwingOff => "CLIMATE_SWING_OFF",
            Self::ClimateSwingBoth => "CLIMATE_SWING_BOTH",
            Self::ClimateSwingVertical => "CLIMATE_SWING_VERTICAL",
            Self::ClimateSwingHorizontal => "CLIMATE_SWING_HORIZONTAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLIMATE_SWING_OFF" => Some(Self::ClimateSwingOff),
            "CLIMATE_SWING_BOTH" => Some(Self::ClimateSwingBoth),
            "CLIMATE_SWING_VERTICAL" => Some(Self::ClimateSwingVertical),
            "CLIMATE_SWING_HORIZONTAL" => Some(Self::ClimateSwingHorizontal),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum ClimateAction {
    #[default]
    Off = 0,
    /// values same as mode for readability
    Cooling = 2,
    Heating = 3,
    Idle = 4,
    Drying = 5,
    Fan = 6,
}
impl ClimateAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Off => "CLIMATE_ACTION_OFF",
            Self::Cooling => "CLIMATE_ACTION_COOLING",
            Self::Heating => "CLIMATE_ACTION_HEATING",
            Self::Idle => "CLIMATE_ACTION_IDLE",
            Self::Drying => "CLIMATE_ACTION_DRYING",
            Self::Fan => "CLIMATE_ACTION_FAN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLIMATE_ACTION_OFF" => Some(Self::Off),
            "CLIMATE_ACTION_COOLING" => Some(Self::Cooling),
            "CLIMATE_ACTION_HEATING" => Some(Self::Heating),
            "CLIMATE_ACTION_IDLE" => Some(Self::Idle),
            "CLIMATE_ACTION_DRYING" => Some(Self::Drying),
            "CLIMATE_ACTION_FAN" => Some(Self::Fan),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum ClimatePreset {
    #[default]
    None = 0,
    Home = 1,
    Away = 2,
    Boost = 3,
    Comfort = 4,
    Eco = 5,
    Sleep = 6,
    Activity = 7,
}
impl ClimatePreset {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "CLIMATE_PRESET_NONE",
            Self::Home => "CLIMATE_PRESET_HOME",
            Self::Away => "CLIMATE_PRESET_AWAY",
            Self::Boost => "CLIMATE_PRESET_BOOST",
            Self::Comfort => "CLIMATE_PRESET_COMFORT",
            Self::Eco => "CLIMATE_PRESET_ECO",
            Self::Sleep => "CLIMATE_PRESET_SLEEP",
            Self::Activity => "CLIMATE_PRESET_ACTIVITY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLIMATE_PRESET_NONE" => Some(Self::None),
            "CLIMATE_PRESET_HOME" => Some(Self::Home),
            "CLIMATE_PRESET_AWAY" => Some(Self::Away),
            "CLIMATE_PRESET_BOOST" => Some(Self::Boost),
            "CLIMATE_PRESET_COMFORT" => Some(Self::Comfort),
            "CLIMATE_PRESET_ECO" => Some(Self::Eco),
            "CLIMATE_PRESET_SLEEP" => Some(Self::Sleep),
            "CLIMATE_PRESET_ACTIVITY" => Some(Self::Activity),
            _ => None,
        }
    }
}
/// ==================== NUMBER ====================
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum NumberMode {
    #[default]
    Auto = 0,
    Box = 1,
    Slider = 2,
}
impl NumberMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Auto => "NUMBER_MODE_AUTO",
            Self::Box => "NUMBER_MODE_BOX",
            Self::Slider => "NUMBER_MODE_SLIDER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NUMBER_MODE_AUTO" => Some(Self::Auto),
            "NUMBER_MODE_BOX" => Some(Self::Box),
            "NUMBER_MODE_SLIDER" => Some(Self::Slider),
            _ => None,
        }
    }
}
/// ==================== LOCK ====================
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum LockState {
    #[default]
    None = 0,
    Locked = 1,
    Unlocked = 2,
    Jammed = 3,
    Locking = 4,
    Unlocking = 5,
}
impl LockState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "LOCK_STATE_NONE",
            Self::Locked => "LOCK_STATE_LOCKED",
            Self::Unlocked => "LOCK_STATE_UNLOCKED",
            Self::Jammed => "LOCK_STATE_JAMMED",
            Self::Locking => "LOCK_STATE_LOCKING",
            Self::Unlocking => "LOCK_STATE_UNLOCKING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOCK_STATE_NONE" => Some(Self::None),
            "LOCK_STATE_LOCKED" => Some(Self::Locked),
            "LOCK_STATE_UNLOCKED" => Some(Self::Unlocked),
            "LOCK_STATE_JAMMED" => Some(Self::Jammed),
            "LOCK_STATE_LOCKING" => Some(Self::Locking),
            "LOCK_STATE_UNLOCKING" => Some(Self::Unlocking),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum LockCommand {
    #[default]
    LockUnlock = 0,
    LockLock = 1,
    LockOpen = 2,
}
impl LockCommand {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::LockUnlock => "LOCK_UNLOCK",
            Self::LockLock => "LOCK_LOCK",
            Self::LockOpen => "LOCK_OPEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOCK_UNLOCK" => Some(Self::LockUnlock),
            "LOCK_LOCK" => Some(Self::LockLock),
            "LOCK_OPEN" => Some(Self::LockOpen),
            _ => None,
        }
    }
}
/// ==================== MEDIA PLAYER ====================
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum MediaPlayerState {
    #[default]
    None = 0,
    Idle = 1,
    Playing = 2,
    Paused = 3,
}
impl MediaPlayerState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "MEDIA_PLAYER_STATE_NONE",
            Self::Idle => "MEDIA_PLAYER_STATE_IDLE",
            Self::Playing => "MEDIA_PLAYER_STATE_PLAYING",
            Self::Paused => "MEDIA_PLAYER_STATE_PAUSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MEDIA_PLAYER_STATE_NONE" => Some(Self::None),
            "MEDIA_PLAYER_STATE_IDLE" => Some(Self::Idle),
            "MEDIA_PLAYER_STATE_PLAYING" => Some(Self::Playing),
            "MEDIA_PLAYER_STATE_PAUSED" => Some(Self::Paused),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum MediaPlayerCommand {
    #[default]
    Play = 0,
    Pause = 1,
    Stop = 2,
    Mute = 3,
    Unmute = 4,
}
impl MediaPlayerCommand {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Play => "MEDIA_PLAYER_COMMAND_PLAY",
            Self::Pause => "MEDIA_PLAYER_COMMAND_PAUSE",
            Self::Stop => "MEDIA_PLAYER_COMMAND_STOP",
            Self::Mute => "MEDIA_PLAYER_COMMAND_MUTE",
            Self::Unmute => "MEDIA_PLAYER_COMMAND_UNMUTE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MEDIA_PLAYER_COMMAND_PLAY" => Some(Self::Play),
            "MEDIA_PLAYER_COMMAND_PAUSE" => Some(Self::Pause),
            "MEDIA_PLAYER_COMMAND_STOP" => Some(Self::Stop),
            "MEDIA_PLAYER_COMMAND_MUTE" => Some(Self::Mute),
            "MEDIA_PLAYER_COMMAND_UNMUTE" => Some(Self::Unmute),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum MediaPlayerFormatPurpose {
    #[default]
    Default = 0,
    Announcement = 1,
}
impl MediaPlayerFormatPurpose {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Default => "MEDIA_PLAYER_FORMAT_PURPOSE_DEFAULT",
            Self::Announcement => "MEDIA_PLAYER_FORMAT_PURPOSE_ANNOUNCEMENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MEDIA_PLAYER_FORMAT_PURPOSE_DEFAULT" => Some(Self::Default),
            "MEDIA_PLAYER_FORMAT_PURPOSE_ANNOUNCEMENT" => Some(Self::Announcement),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum BluetoothDeviceRequestType {
    #[default]
    Connect = 0,
    Disconnect = 1,
    Pair = 2,
    Unpair = 3,
    ConnectV3WithCache = 4,
    ConnectV3WithoutCache = 5,
    ClearCache = 6,
}
impl BluetoothDeviceRequestType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Connect => "BLUETOOTH_DEVICE_REQUEST_TYPE_CONNECT",
            Self::Disconnect => "BLUETOOTH_DEVICE_REQUEST_TYPE_DISCONNECT",
            Self::Pair => "BLUETOOTH_DEVICE_REQUEST_TYPE_PAIR",
            Self::Unpair => "BLUETOOTH_DEVICE_REQUEST_TYPE_UNPAIR",
            Self::ConnectV3WithCache => {
                "BLUETOOTH_DEVICE_REQUEST_TYPE_CONNECT_V3_WITH_CACHE"
            }
            Self::ConnectV3WithoutCache => {
                "BLUETOOTH_DEVICE_REQUEST_TYPE_CONNECT_V3_WITHOUT_CACHE"
            }
            Self::ClearCache => "BLUETOOTH_DEVICE_REQUEST_TYPE_CLEAR_CACHE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLUETOOTH_DEVICE_REQUEST_TYPE_CONNECT" => Some(Self::Connect),
            "BLUETOOTH_DEVICE_REQUEST_TYPE_DISCONNECT" => Some(Self::Disconnect),
            "BLUETOOTH_DEVICE_REQUEST_TYPE_PAIR" => Some(Self::Pair),
            "BLUETOOTH_DEVICE_REQUEST_TYPE_UNPAIR" => Some(Self::Unpair),
            "BLUETOOTH_DEVICE_REQUEST_TYPE_CONNECT_V3_WITH_CACHE" => {
                Some(Self::ConnectV3WithCache)
            }
            "BLUETOOTH_DEVICE_REQUEST_TYPE_CONNECT_V3_WITHOUT_CACHE" => {
                Some(Self::ConnectV3WithoutCache)
            }
            "BLUETOOTH_DEVICE_REQUEST_TYPE_CLEAR_CACHE" => Some(Self::ClearCache),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum BluetoothScannerState {
    #[default]
    Idle = 0,
    Starting = 1,
    Running = 2,
    Failed = 3,
    Stopping = 4,
    Stopped = 5,
}
impl BluetoothScannerState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Idle => "BLUETOOTH_SCANNER_STATE_IDLE",
            Self::Starting => "BLUETOOTH_SCANNER_STATE_STARTING",
            Self::Running => "BLUETOOTH_SCANNER_STATE_RUNNING",
            Self::Failed => "BLUETOOTH_SCANNER_STATE_FAILED",
            Self::Stopping => "BLUETOOTH_SCANNER_STATE_STOPPING",
            Self::Stopped => "BLUETOOTH_SCANNER_STATE_STOPPED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLUETOOTH_SCANNER_STATE_IDLE" => Some(Self::Idle),
            "BLUETOOTH_SCANNER_STATE_STARTING" => Some(Self::Starting),
            "BLUETOOTH_SCANNER_STATE_RUNNING" => Some(Self::Running),
            "BLUETOOTH_SCANNER_STATE_FAILED" => Some(Self::Failed),
            "BLUETOOTH_SCANNER_STATE_STOPPING" => Some(Self::Stopping),
            "BLUETOOTH_SCANNER_STATE_STOPPED" => Some(Self::Stopped),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum BluetoothScannerMode {
    #[default]
    Passive = 0,
    Active = 1,
}
impl BluetoothScannerMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Passive => "BLUETOOTH_SCANNER_MODE_PASSIVE",
            Self::Active => "BLUETOOTH_SCANNER_MODE_ACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLUETOOTH_SCANNER_MODE_PASSIVE" => Some(Self::Passive),
            "BLUETOOTH_SCANNER_MODE_ACTIVE" => Some(Self::Active),
            _ => None,
        }
    }
}
/// ==================== VOICE ASSISTANT ====================
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum VoiceAssistantSubscribeFlag {
    #[default]
    VoiceAssistantSubscribeNone = 0,
    VoiceAssistantSubscribeApiAudio = 1,
}
impl VoiceAssistantSubscribeFlag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::VoiceAssistantSubscribeNone => "VOICE_ASSISTANT_SUBSCRIBE_NONE",
            Self::VoiceAssistantSubscribeApiAudio => {
                "VOICE_ASSISTANT_SUBSCRIBE_API_AUDIO"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VOICE_ASSISTANT_SUBSCRIBE_NONE" => Some(Self::VoiceAssistantSubscribeNone),
            "VOICE_ASSISTANT_SUBSCRIBE_API_AUDIO" => {
                Some(Self::VoiceAssistantSubscribeApiAudio)
            }
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum VoiceAssistantRequestFlag {
    #[default]
    VoiceAssistantRequestNone = 0,
    VoiceAssistantRequestUseVad = 1,
    VoiceAssistantRequestUseWakeWord = 2,
}
impl VoiceAssistantRequestFlag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::VoiceAssistantRequestNone => "VOICE_ASSISTANT_REQUEST_NONE",
            Self::VoiceAssistantRequestUseVad => "VOICE_ASSISTANT_REQUEST_USE_VAD",
            Self::VoiceAssistantRequestUseWakeWord => {
                "VOICE_ASSISTANT_REQUEST_USE_WAKE_WORD"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VOICE_ASSISTANT_REQUEST_NONE" => Some(Self::VoiceAssistantRequestNone),
            "VOICE_ASSISTANT_REQUEST_USE_VAD" => Some(Self::VoiceAssistantRequestUseVad),
            "VOICE_ASSISTANT_REQUEST_USE_WAKE_WORD" => {
                Some(Self::VoiceAssistantRequestUseWakeWord)
            }
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum VoiceAssistantEvent {
    #[default]
    VoiceAssistantError = 0,
    VoiceAssistantRunStart = 1,
    VoiceAssistantRunEnd = 2,
    VoiceAssistantSttStart = 3,
    VoiceAssistantSttEnd = 4,
    VoiceAssistantIntentStart = 5,
    VoiceAssistantIntentEnd = 6,
    VoiceAssistantTtsStart = 7,
    VoiceAssistantTtsEnd = 8,
    VoiceAssistantWakeWordStart = 9,
    VoiceAssistantWakeWordEnd = 10,
    VoiceAssistantSttVadStart = 11,
    VoiceAssistantSttVadEnd = 12,
    VoiceAssistantTtsStreamStart = 98,
    VoiceAssistantTtsStreamEnd = 99,
}
impl VoiceAssistantEvent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::VoiceAssistantError => "VOICE_ASSISTANT_ERROR",
            Self::VoiceAssistantRunStart => "VOICE_ASSISTANT_RUN_START",
            Self::VoiceAssistantRunEnd => "VOICE_ASSISTANT_RUN_END",
            Self::VoiceAssistantSttStart => "VOICE_ASSISTANT_STT_START",
            Self::VoiceAssistantSttEnd => "VOICE_ASSISTANT_STT_END",
            Self::VoiceAssistantIntentStart => "VOICE_ASSISTANT_INTENT_START",
            Self::VoiceAssistantIntentEnd => "VOICE_ASSISTANT_INTENT_END",
            Self::VoiceAssistantTtsStart => "VOICE_ASSISTANT_TTS_START",
            Self::VoiceAssistantTtsEnd => "VOICE_ASSISTANT_TTS_END",
            Self::VoiceAssistantWakeWordStart => "VOICE_ASSISTANT_WAKE_WORD_START",
            Self::VoiceAssistantWakeWordEnd => "VOICE_ASSISTANT_WAKE_WORD_END",
            Self::VoiceAssistantSttVadStart => "VOICE_ASSISTANT_STT_VAD_START",
            Self::VoiceAssistantSttVadEnd => "VOICE_ASSISTANT_STT_VAD_END",
            Self::VoiceAssistantTtsStreamStart => "VOICE_ASSISTANT_TTS_STREAM_START",
            Self::VoiceAssistantTtsStreamEnd => "VOICE_ASSISTANT_TTS_STREAM_END",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VOICE_ASSISTANT_ERROR" => Some(Self::VoiceAssistantError),
            "VOICE_ASSISTANT_RUN_START" => Some(Self::VoiceAssistantRunStart),
            "VOICE_ASSISTANT_RUN_END" => Some(Self::VoiceAssistantRunEnd),
            "VOICE_ASSISTANT_STT_START" => Some(Self::VoiceAssistantSttStart),
            "VOICE_ASSISTANT_STT_END" => Some(Self::VoiceAssistantSttEnd),
            "VOICE_ASSISTANT_INTENT_START" => Some(Self::VoiceAssistantIntentStart),
            "VOICE_ASSISTANT_INTENT_END" => Some(Self::VoiceAssistantIntentEnd),
            "VOICE_ASSISTANT_TTS_START" => Some(Self::VoiceAssistantTtsStart),
            "VOICE_ASSISTANT_TTS_END" => Some(Self::VoiceAssistantTtsEnd),
            "VOICE_ASSISTANT_WAKE_WORD_START" => Some(Self::VoiceAssistantWakeWordStart),
            "VOICE_ASSISTANT_WAKE_WORD_END" => Some(Self::VoiceAssistantWakeWordEnd),
            "VOICE_ASSISTANT_STT_VAD_START" => Some(Self::VoiceAssistantSttVadStart),
            "VOICE_ASSISTANT_STT_VAD_END" => Some(Self::VoiceAssistantSttVadEnd),
            "VOICE_ASSISTANT_TTS_STREAM_START" => {
                Some(Self::VoiceAssistantTtsStreamStart)
            }
            "VOICE_ASSISTANT_TTS_STREAM_END" => Some(Self::VoiceAssistantTtsStreamEnd),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum VoiceAssistantTimerEvent {
    #[default]
    VoiceAssistantTimerStarted = 0,
    VoiceAssistantTimerUpdated = 1,
    VoiceAssistantTimerCancelled = 2,
    VoiceAssistantTimerFinished = 3,
}
impl VoiceAssistantTimerEvent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::VoiceAssistantTimerStarted => "VOICE_ASSISTANT_TIMER_STARTED",
            Self::VoiceAssistantTimerUpdated => "VOICE_ASSISTANT_TIMER_UPDATED",
            Self::VoiceAssistantTimerCancelled => "VOICE_ASSISTANT_TIMER_CANCELLED",
            Self::VoiceAssistantTimerFinished => "VOICE_ASSISTANT_TIMER_FINISHED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VOICE_ASSISTANT_TIMER_STARTED" => Some(Self::VoiceAssistantTimerStarted),
            "VOICE_ASSISTANT_TIMER_UPDATED" => Some(Self::VoiceAssistantTimerUpdated),
            "VOICE_ASSISTANT_TIMER_CANCELLED" => Some(Self::VoiceAssistantTimerCancelled),
            "VOICE_ASSISTANT_TIMER_FINISHED" => Some(Self::VoiceAssistantTimerFinished),
            _ => None,
        }
    }
}
/// ==================== ALARM CONTROL PANEL ====================
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum AlarmControlPanelState {
    #[default]
    AlarmStateDisarmed = 0,
    AlarmStateArmedHome = 1,
    AlarmStateArmedAway = 2,
    AlarmStateArmedNight = 3,
    AlarmStateArmedVacation = 4,
    AlarmStateArmedCustomBypass = 5,
    AlarmStatePending = 6,
    AlarmStateArming = 7,
    AlarmStateDisarming = 8,
    AlarmStateTriggered = 9,
}
impl AlarmControlPanelState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::AlarmStateDisarmed => "ALARM_STATE_DISARMED",
            Self::AlarmStateArmedHome => "ALARM_STATE_ARMED_HOME",
            Self::AlarmStateArmedAway => "ALARM_STATE_ARMED_AWAY",
            Self::AlarmStateArmedNight => "ALARM_STATE_ARMED_NIGHT",
            Self::AlarmStateArmedVacation => "ALARM_STATE_ARMED_VACATION",
            Self::AlarmStateArmedCustomBypass => "ALARM_STATE_ARMED_CUSTOM_BYPASS",
            Self::AlarmStatePending => "ALARM_STATE_PENDING",
            Self::AlarmStateArming => "ALARM_STATE_ARMING",
            Self::AlarmStateDisarming => "ALARM_STATE_DISARMING",
            Self::AlarmStateTriggered => "ALARM_STATE_TRIGGERED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALARM_STATE_DISARMED" => Some(Self::AlarmStateDisarmed),
            "ALARM_STATE_ARMED_HOME" => Some(Self::AlarmStateArmedHome),
            "ALARM_STATE_ARMED_AWAY" => Some(Self::AlarmStateArmedAway),
            "ALARM_STATE_ARMED_NIGHT" => Some(Self::AlarmStateArmedNight),
            "ALARM_STATE_ARMED_VACATION" => Some(Self::AlarmStateArmedVacation),
            "ALARM_STATE_ARMED_CUSTOM_BYPASS" => Some(Self::AlarmStateArmedCustomBypass),
            "ALARM_STATE_PENDING" => Some(Self::AlarmStatePending),
            "ALARM_STATE_ARMING" => Some(Self::AlarmStateArming),
            "ALARM_STATE_DISARMING" => Some(Self::AlarmStateDisarming),
            "ALARM_STATE_TRIGGERED" => Some(Self::AlarmStateTriggered),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum AlarmControlPanelStateCommand {
    #[default]
    AlarmControlPanelDisarm = 0,
    AlarmControlPanelArmAway = 1,
    AlarmControlPanelArmHome = 2,
    AlarmControlPanelArmNight = 3,
    AlarmControlPanelArmVacation = 4,
    AlarmControlPanelArmCustomBypass = 5,
    AlarmControlPanelTrigger = 6,
}
impl AlarmControlPanelStateCommand {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::AlarmControlPanelDisarm => "ALARM_CONTROL_PANEL_DISARM",
            Self::AlarmControlPanelArmAway => "ALARM_CONTROL_PANEL_ARM_AWAY",
            Self::AlarmControlPanelArmHome => "ALARM_CONTROL_PANEL_ARM_HOME",
            Self::AlarmControlPanelArmNight => "ALARM_CONTROL_PANEL_ARM_NIGHT",
            Self::AlarmControlPanelArmVacation => "ALARM_CONTROL_PANEL_ARM_VACATION",
            Self::AlarmControlPanelArmCustomBypass => {
                "ALARM_CONTROL_PANEL_ARM_CUSTOM_BYPASS"
            }
            Self::AlarmControlPanelTrigger => "ALARM_CONTROL_PANEL_TRIGGER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALARM_CONTROL_PANEL_DISARM" => Some(Self::AlarmControlPanelDisarm),
            "ALARM_CONTROL_PANEL_ARM_AWAY" => Some(Self::AlarmControlPanelArmAway),
            "ALARM_CONTROL_PANEL_ARM_HOME" => Some(Self::AlarmControlPanelArmHome),
            "ALARM_CONTROL_PANEL_ARM_NIGHT" => Some(Self::AlarmControlPanelArmNight),
            "ALARM_CONTROL_PANEL_ARM_VACATION" => {
                Some(Self::AlarmControlPanelArmVacation)
            }
            "ALARM_CONTROL_PANEL_ARM_CUSTOM_BYPASS" => {
                Some(Self::AlarmControlPanelArmCustomBypass)
            }
            "ALARM_CONTROL_PANEL_TRIGGER" => Some(Self::AlarmControlPanelTrigger),
            _ => None,
        }
    }
}
/// ===================== TEXT =====================
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum TextMode {
    #[default]
    Text = 0,
    Password = 1,
}
impl TextMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Text => "TEXT_MODE_TEXT",
            Self::Password => "TEXT_MODE_PASSWORD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TEXT_MODE_TEXT" => Some(Self::Text),
            "TEXT_MODE_PASSWORD" => Some(Self::Password),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum ValveOperation {
    #[default]
    Idle = 0,
    IsOpening = 1,
    IsClosing = 2,
}
impl ValveOperation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Idle => "VALVE_OPERATION_IDLE",
            Self::IsOpening => "VALVE_OPERATION_IS_OPENING",
            Self::IsClosing => "VALVE_OPERATION_IS_CLOSING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VALVE_OPERATION_IDLE" => Some(Self::Idle),
            "VALVE_OPERATION_IS_OPENING" => Some(Self::IsOpening),
            "VALVE_OPERATION_IS_CLOSING" => Some(Self::IsClosing),
            _ => None,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum UpdateCommand {
    #[default]
    None = 0,
    Update = 1,
    Check = 2,
}
impl UpdateCommand {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "UPDATE_COMMAND_NONE",
            Self::Update => "UPDATE_COMMAND_UPDATE",
            Self::Check => "UPDATE_COMMAND_CHECK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UPDATE_COMMAND_NONE" => Some(Self::None),
            "UPDATE_COMMAND_UPDATE" => Some(Self::Update),
            "UPDATE_COMMAND_CHECK" => Some(Self::Check),
            _ => None,
        }
    }
}
