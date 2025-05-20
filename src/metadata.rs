#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageSource {
    Client,
    Server,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    // Base packets
    HelloRequest = 1,
    HelloResponse = 2,
    ConnectRequest = 3,
    ConnectResponse = 4,
    DisconnectRequest = 5,
    DisconnectResponse = 6,
    PingRequest = 7,
    PingResponse = 8,
    DeviceInfoRequest = 9,
    DeviceInfoResponse = 10,

    // List entities
    ListEntitiesRequest = 11,
    ListEntitiesBinarySensorResponse = 12,
    ListEntitiesCoverResponse = 13,
    ListEntitiesFanResponse = 14,
    ListEntitiesLightResponse = 15,
    ListEntitiesSensorResponse = 16,
    ListEntitiesSwitchResponse = 17,
    ListEntitiesTextSensorResponse = 18,
    ListEntitiesDoneResponse = 19,

    // Subscribe states
    SubscribeStatesRequest = 20,
    BinarySensorStateResponse = 21,
    CoverStateResponse = 22,
    FanStateResponse = 23,
    LightStateResponse = 24,
    SensorStateResponse = 25,
    SwitchStateResponse = 26,
    TextSensorStateResponse = 27,

    // Subscribe logs
    SubscribeLogsRequest = 28,
    SubscribeLogsResponse = 29,

    // Cover commands
    CoverCommandRequest = 30,

    // Fan commands
    FanCommandRequest = 31,

    // Light commands
    LightCommandRequest = 32,

    // Switch commands
    SwitchCommandRequest = 33,

    // Homeassistant services
    SubscribeHomeassistantServicesRequest = 34,
    HomeassistantServiceResponse = 35,

    // Get time
    GetTimeRequest = 36,
    GetTimeResponse = 37,

    // Subscribe Home Assistant states
    SubscribeHomeAssistantStatesRequest = 38,
    SubscribeHomeAssistantStateResponse = 39,
    HomeAssistantStateResponse = 40,

    // Service calls
    ListEntitiesServicesResponse = 41,
    ExecuteServiceRequest = 42,

    // Camera
    ListEntitiesCameraResponse = 43,
    CameraImageResponse = 44,
    CameraImageRequest = 45,

    // Climate
    ListEntitiesClimateResponse = 46,
    ClimateStateResponse = 47,
    ClimateCommandRequest = 48,

    // Number
    ListEntitiesNumberResponse = 49,
    NumberStateResponse = 50,
    NumberCommandRequest = 51,

    // Select
    ListEntitiesSelectResponse = 52,
    SelectStateResponse = 53,
    SelectCommandRequest = 54,

    // Siren
    ListEntitiesSirenResponse = 55,
    SirenStateResponse = 56,
    SirenCommandRequest = 57,

    // Lock
    ListEntitiesLockResponse = 58,
    LockStateResponse = 59,
    LockCommandRequest = 60,

    // Button
    ListEntitiesButtonResponse = 61,
    ButtonCommandRequest = 62,

    // Media Player
    ListEntitiesMediaPlayerResponse = 63,
    MediaPlayerStateResponse = 64,
    MediaPlayerCommandRequest = 65,

    // Bluetooth
    SubscribeBluetoothLEAdvertisementsRequest = 66,
    BluetoothLEAdvertisementResponse = 67,
    BluetoothDeviceRequest = 68,
    BluetoothDeviceConnectionResponse = 69,
    BluetoothGATTGetServicesRequest = 70,
    BluetoothGATTGetServicesResponse = 71,
    BluetoothGATTGetServicesDoneResponse = 72,
    BluetoothGATTReadRequest = 73,
    BluetoothGATTReadResponse = 74,
    BluetoothGATTWriteRequest = 75,
    BluetoothGATTReadDescriptorRequest = 76,
    BluetoothGATTWriteDescriptorRequest = 77,
    BluetoothGATTNotifyRequest = 78,
    BluetoothGATTNotifyDataResponse = 79,
    SubscribeBluetoothConnectionsFreeRequest = 80,
    BluetoothConnectionsFreeResponse = 81,
    BluetoothGATTErrorResponse = 82,
    BluetoothGATTWriteResponse = 83,
    BluetoothGATTNotifyResponse = 84,
    BluetoothDevicePairingResponse = 85,
    BluetoothDeviceUnpairingResponse = 86,
    UnsubscribeBluetoothLEAdvertisementsRequest = 87,
    BluetoothDeviceClearCacheResponse = 88,

    // Voice Assistant
    SubscribeVoiceAssistantRequest = 89,
    VoiceAssistantRequest = 90,
    VoiceAssistantResponse = 91,
    VoiceAssistantEventResponse = 92,
    BluetoothLERawAdvertisementsResponse = 93,

    // Alarm Control Panel
    ListEntitiesAlarmControlPanelResponse = 94,
    AlarmControlPanelStateResponse = 95,
    AlarmControlPanelCommandRequest = 96,

    // Text
    ListEntitiesTextResponse = 97,
    TextStateResponse = 98,
    TextCommandRequest = 99,

    // DateTime
    ListEntitiesDateResponse = 100,
    DateStateResponse = 101,
    DateCommandRequest = 102,
    ListEntitiesTimeResponse = 103,
    TimeStateResponse = 104,
    TimeCommandRequest = 105,
    VoiceAssistantAudio = 106,

    // Event
    ListEntitiesEventResponse = 107,
    EventResponse = 108,

    // Valve
    ListEntitiesValveResponse = 109,
    ValveStateResponse = 110,
    ValveCommandRequest = 111,

    // DateTime (continued)
    ListEntitiesDateTimeResponse = 112,
    DateTimeStateResponse = 113,
    DateTimeCommandRequest = 114,

    // Voice Assistant (continued)
    VoiceAssistantTimerEventResponse = 115,

    // Update
    ListEntitiesUpdateResponse = 116,
    UpdateStateResponse = 117,
    UpdateCommandRequest = 118,

    // Voice Assistant (continued)
    VoiceAssistantAnnounceRequest = 119,
    VoiceAssistantAnnounceFinished = 120,
    VoiceAssistantConfigurationRequest = 121,
    VoiceAssistantConfigurationResponse = 122,
    VoiceAssistantSetConfiguration = 123,

    // API Noise
    NoiseEncryptionSetKeyRequest = 124,
    NoiseEncryptionSetKeyResponse = 125,

    // Bluetooth (continued)
    BluetoothScannerStateResponse = 126,
    BluetoothScannerSetModeRequest = 127,
}

impl MessageType {
    /// Convert a u8 to a MessageType
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::HelloRequest),
            2 => Some(Self::HelloResponse),
            3 => Some(Self::ConnectRequest),
            4 => Some(Self::ConnectResponse),
            5 => Some(Self::DisconnectRequest),
            6 => Some(Self::DisconnectResponse),
            7 => Some(Self::PingRequest),
            8 => Some(Self::PingResponse),
            9 => Some(Self::DeviceInfoRequest),
            10 => Some(Self::DeviceInfoResponse),
            11 => Some(Self::ListEntitiesRequest),
            12 => Some(Self::ListEntitiesBinarySensorResponse),
            13 => Some(Self::ListEntitiesCoverResponse),
            14 => Some(Self::ListEntitiesFanResponse),
            15 => Some(Self::ListEntitiesLightResponse),
            16 => Some(Self::ListEntitiesSensorResponse),
            17 => Some(Self::ListEntitiesSwitchResponse),
            18 => Some(Self::ListEntitiesTextSensorResponse),
            19 => Some(Self::ListEntitiesDoneResponse),
            20 => Some(Self::SubscribeStatesRequest),
            21 => Some(Self::BinarySensorStateResponse),
            22 => Some(Self::CoverStateResponse),
            23 => Some(Self::FanStateResponse),
            24 => Some(Self::LightStateResponse),
            25 => Some(Self::SensorStateResponse),
            26 => Some(Self::SwitchStateResponse),
            27 => Some(Self::TextSensorStateResponse),
            28 => Some(Self::SubscribeLogsRequest),
            29 => Some(Self::SubscribeLogsResponse),
            30 => Some(Self::CoverCommandRequest),
            31 => Some(Self::FanCommandRequest),
            32 => Some(Self::LightCommandRequest),
            33 => Some(Self::SwitchCommandRequest),
            34 => Some(Self::SubscribeHomeassistantServicesRequest),
            35 => Some(Self::HomeassistantServiceResponse),
            36 => Some(Self::GetTimeRequest),
            37 => Some(Self::GetTimeResponse),
            38 => Some(Self::SubscribeHomeAssistantStatesRequest),
            39 => Some(Self::SubscribeHomeAssistantStateResponse),
            40 => Some(Self::HomeAssistantStateResponse),
            41 => Some(Self::ListEntitiesServicesResponse),
            42 => Some(Self::ExecuteServiceRequest),
            43 => Some(Self::ListEntitiesCameraResponse),
            44 => Some(Self::CameraImageResponse),
            45 => Some(Self::CameraImageRequest),
            46 => Some(Self::ListEntitiesClimateResponse),
            47 => Some(Self::ClimateStateResponse),
            48 => Some(Self::ClimateCommandRequest),
            49 => Some(Self::ListEntitiesNumberResponse),
            50 => Some(Self::NumberStateResponse),
            51 => Some(Self::NumberCommandRequest),
            52 => Some(Self::ListEntitiesSelectResponse),
            53 => Some(Self::SelectStateResponse),
            54 => Some(Self::SelectCommandRequest),
            55 => Some(Self::ListEntitiesSirenResponse),
            56 => Some(Self::SirenStateResponse),
            57 => Some(Self::SirenCommandRequest),
            58 => Some(Self::ListEntitiesLockResponse),
            59 => Some(Self::LockStateResponse),
            60 => Some(Self::LockCommandRequest),
            61 => Some(Self::ListEntitiesButtonResponse),
            62 => Some(Self::ButtonCommandRequest),
            63 => Some(Self::ListEntitiesMediaPlayerResponse),
            64 => Some(Self::MediaPlayerStateResponse),
            65 => Some(Self::MediaPlayerCommandRequest),
            66 => Some(Self::SubscribeBluetoothLEAdvertisementsRequest),
            67 => Some(Self::BluetoothLEAdvertisementResponse),
            68 => Some(Self::BluetoothDeviceRequest),
            69 => Some(Self::BluetoothDeviceConnectionResponse),
            70 => Some(Self::BluetoothGATTGetServicesRequest),
            71 => Some(Self::BluetoothGATTGetServicesResponse),
            72 => Some(Self::BluetoothGATTGetServicesDoneResponse),
            73 => Some(Self::BluetoothGATTReadRequest),
            74 => Some(Self::BluetoothGATTReadResponse),
            75 => Some(Self::BluetoothGATTWriteRequest),
            76 => Some(Self::BluetoothGATTReadDescriptorRequest),
            77 => Some(Self::BluetoothGATTWriteDescriptorRequest),
            78 => Some(Self::BluetoothGATTNotifyRequest),
            79 => Some(Self::BluetoothGATTNotifyDataResponse),
            80 => Some(Self::SubscribeBluetoothConnectionsFreeRequest),
            81 => Some(Self::BluetoothConnectionsFreeResponse),
            82 => Some(Self::BluetoothGATTErrorResponse),
            83 => Some(Self::BluetoothGATTWriteResponse),
            84 => Some(Self::BluetoothGATTNotifyResponse),
            85 => Some(Self::BluetoothDevicePairingResponse),
            86 => Some(Self::BluetoothDeviceUnpairingResponse),
            87 => Some(Self::UnsubscribeBluetoothLEAdvertisementsRequest),
            88 => Some(Self::BluetoothDeviceClearCacheResponse),
            89 => Some(Self::SubscribeVoiceAssistantRequest),
            90 => Some(Self::VoiceAssistantRequest),
            91 => Some(Self::VoiceAssistantResponse),
            92 => Some(Self::VoiceAssistantEventResponse),
            93 => Some(Self::BluetoothLERawAdvertisementsResponse),
            94 => Some(Self::ListEntitiesAlarmControlPanelResponse),
            95 => Some(Self::AlarmControlPanelStateResponse),
            96 => Some(Self::AlarmControlPanelCommandRequest),
            97 => Some(Self::ListEntitiesTextResponse),
            98 => Some(Self::TextStateResponse),
            99 => Some(Self::TextCommandRequest),
            100 => Some(Self::ListEntitiesDateResponse),
            101 => Some(Self::DateStateResponse),
            102 => Some(Self::DateCommandRequest),
            103 => Some(Self::ListEntitiesTimeResponse),
            104 => Some(Self::TimeStateResponse),
            105 => Some(Self::TimeCommandRequest),
            106 => Some(Self::VoiceAssistantAudio),
            107 => Some(Self::ListEntitiesEventResponse),
            108 => Some(Self::EventResponse),
            109 => Some(Self::ListEntitiesValveResponse),
            110 => Some(Self::ValveStateResponse),
            111 => Some(Self::ValveCommandRequest),
            112 => Some(Self::ListEntitiesDateTimeResponse),
            113 => Some(Self::DateTimeStateResponse),
            114 => Some(Self::DateTimeCommandRequest),
            115 => Some(Self::VoiceAssistantTimerEventResponse),
            116 => Some(Self::ListEntitiesUpdateResponse),
            117 => Some(Self::UpdateStateResponse),
            118 => Some(Self::UpdateCommandRequest),
            119 => Some(Self::VoiceAssistantAnnounceRequest),
            120 => Some(Self::VoiceAssistantAnnounceFinished),
            121 => Some(Self::VoiceAssistantConfigurationRequest),
            122 => Some(Self::VoiceAssistantConfigurationResponse),
            123 => Some(Self::VoiceAssistantSetConfiguration),
            124 => Some(Self::NoiseEncryptionSetKeyRequest),
            125 => Some(Self::NoiseEncryptionSetKeyResponse),
            126 => Some(Self::BluetoothScannerStateResponse),
            127 => Some(Self::BluetoothScannerSetModeRequest),
            _ => None,
        }
    }

    /// Get the source of this message type (client, server, or both)
    pub fn source(&self) -> MessageSource {
        match self {
            // Client sources
            Self::HelloRequest | Self::ConnectRequest | Self::DeviceInfoRequest |
            Self::ListEntitiesRequest | Self::SubscribeStatesRequest |
            Self::SubscribeLogsRequest | Self::CoverCommandRequest |
            Self::FanCommandRequest | Self::LightCommandRequest |
            Self::SwitchCommandRequest | Self::SubscribeHomeassistantServicesRequest |
            Self::SubscribeHomeAssistantStatesRequest | Self::ExecuteServiceRequest |
            Self::CameraImageRequest | Self::ClimateCommandRequest |
            Self::NumberCommandRequest | Self::SelectCommandRequest |
            Self::SirenCommandRequest | Self::LockCommandRequest |
            Self::ButtonCommandRequest | Self::MediaPlayerCommandRequest |
            Self::SubscribeBluetoothLEAdvertisementsRequest | Self::BluetoothDeviceRequest |
            Self::BluetoothGATTGetServicesRequest | Self::BluetoothGATTReadRequest |
            Self::BluetoothGATTWriteRequest | Self::BluetoothGATTReadDescriptorRequest |
            Self::BluetoothGATTWriteDescriptorRequest | Self::BluetoothGATTNotifyRequest |
            Self::SubscribeBluetoothConnectionsFreeRequest |
            Self::UnsubscribeBluetoothLEAdvertisementsRequest |
            Self::SubscribeVoiceAssistantRequest | Self::VoiceAssistantResponse |
            Self::VoiceAssistantEventResponse | Self::AlarmControlPanelCommandRequest |
            Self::TextCommandRequest | Self::DateCommandRequest |
            Self::TimeCommandRequest | Self::ValveCommandRequest |
            Self::DateTimeCommandRequest | Self::VoiceAssistantTimerEventResponse |
            Self::UpdateCommandRequest | Self::VoiceAssistantAnnounceRequest |
            Self::VoiceAssistantConfigurationRequest | Self::VoiceAssistantSetConfiguration |
            Self::HomeAssistantStateResponse | Self::NoiseEncryptionSetKeyRequest |
            Self::BluetoothScannerSetModeRequest => MessageSource::Client,

            // Server sources
            Self::HelloResponse | Self::ConnectResponse | Self::DeviceInfoResponse |
            Self::ListEntitiesBinarySensorResponse | Self::ListEntitiesCoverResponse |
            Self::ListEntitiesFanResponse | Self::ListEntitiesLightResponse |
            Self::ListEntitiesSensorResponse | Self::ListEntitiesSwitchResponse |
            Self::ListEntitiesTextSensorResponse | Self::ListEntitiesDoneResponse |
            Self::BinarySensorStateResponse | Self::CoverStateResponse |
            Self::FanStateResponse | Self::LightStateResponse |
            Self::SensorStateResponse | Self::SwitchStateResponse |
            Self::TextSensorStateResponse | Self::SubscribeLogsResponse |
            Self::HomeassistantServiceResponse | Self::SubscribeHomeAssistantStateResponse |
            Self::ListEntitiesServicesResponse | Self::ListEntitiesCameraResponse |
            Self::CameraImageResponse | Self::ListEntitiesClimateResponse |
            Self::ClimateStateResponse | Self::ListEntitiesNumberResponse |
            Self::NumberStateResponse | Self::ListEntitiesSelectResponse |
            Self::SelectStateResponse | Self::ListEntitiesSirenResponse |
            Self::SirenStateResponse | Self::ListEntitiesLockResponse |
            Self::LockStateResponse | Self::ListEntitiesButtonResponse |
            Self::ListEntitiesMediaPlayerResponse | Self::MediaPlayerStateResponse |
            Self::BluetoothLEAdvertisementResponse | Self::BluetoothDeviceConnectionResponse |
            Self::BluetoothGATTGetServicesResponse | Self::BluetoothGATTGetServicesDoneResponse |
            Self::BluetoothGATTReadResponse | Self::BluetoothGATTNotifyDataResponse |
            Self::BluetoothConnectionsFreeResponse | Self::BluetoothGATTErrorResponse |
            Self::BluetoothGATTWriteResponse | Self::BluetoothGATTNotifyResponse |
            Self::BluetoothDevicePairingResponse | Self::BluetoothDeviceUnpairingResponse |
            Self::BluetoothDeviceClearCacheResponse | Self::VoiceAssistantRequest |
            Self::BluetoothLERawAdvertisementsResponse | Self::ListEntitiesAlarmControlPanelResponse |
            Self::AlarmControlPanelStateResponse | Self::ListEntitiesTextResponse |
            Self::TextStateResponse | Self::ListEntitiesDateResponse |
            Self::DateStateResponse | Self::ListEntitiesTimeResponse |
            Self::TimeStateResponse | Self::ListEntitiesEventResponse |
            Self::EventResponse | Self::ListEntitiesValveResponse |
            Self::ValveStateResponse | Self::ListEntitiesDateTimeResponse |
            Self::DateTimeStateResponse | Self::ListEntitiesUpdateResponse |
            Self::UpdateStateResponse | Self::VoiceAssistantAnnounceFinished |
            Self::VoiceAssistantConfigurationResponse | Self::NoiseEncryptionSetKeyResponse |
            Self::BluetoothScannerStateResponse => MessageSource::Server,

            // Both sources
            Self::DisconnectRequest | Self::DisconnectResponse |
            Self::PingRequest | Self::PingResponse |
            Self::GetTimeRequest | Self::GetTimeResponse |
            Self::VoiceAssistantAudio => MessageSource::Both,
        }
    }

    /// Check if the message has the no_delay option
    pub fn has_no_delay(&self) -> bool {
        match self {
            Self::HelloRequest | Self::HelloResponse |
            Self::ConnectRequest | Self::ConnectResponse |
            Self::DisconnectRequest | Self::DisconnectResponse |
            Self::ListEntitiesDoneResponse |
            Self::BinarySensorStateResponse | Self::CoverStateResponse |
            Self::FanStateResponse | Self::LightStateResponse |
            Self::SensorStateResponse | Self::SwitchStateResponse |
            Self::TextSensorStateResponse | Self::HomeassistantServiceResponse |
            Self::GetTimeResponse | Self::CoverCommandRequest |
            Self::FanCommandRequest | Self::LightCommandRequest |
            Self::SwitchCommandRequest | Self::ExecuteServiceRequest |
            Self::CameraImageRequest | Self::ClimateStateResponse |
            Self::ClimateCommandRequest | Self::NumberStateResponse |
            Self::NumberCommandRequest | Self::SelectStateResponse |
            Self::SelectCommandRequest | Self::SirenStateResponse |
            Self::SirenCommandRequest | Self::LockStateResponse |
            Self::LockCommandRequest | Self::ButtonCommandRequest |
            Self::MediaPlayerStateResponse | Self::MediaPlayerCommandRequest |
            Self::BluetoothLEAdvertisementResponse | Self::BluetoothLERawAdvertisementsResponse |
            Self::AlarmControlPanelStateResponse | Self::AlarmControlPanelCommandRequest |
            Self::TextStateResponse | Self::TextCommandRequest |
            Self::DateStateResponse | Self::DateCommandRequest |
            Self::TimeStateResponse | Self::TimeCommandRequest |
            Self::ValveStateResponse | Self::ValveCommandRequest |
            Self::DateTimeStateResponse | Self::DateTimeCommandRequest |
            Self::HomeAssistantStateResponse | Self::UpdateStateResponse |
            Self::UpdateCommandRequest => true,

            _ => false,
        }
    }

    /// Check if the message needs a setup connection
    pub fn needs_setup_connection(&self) -> bool {
        match self {
            Self::HelloRequest | Self::HelloResponse |
            Self::ConnectRequest | Self::ConnectResponse |
            Self::DisconnectRequest | Self::DisconnectResponse |
            Self::PingRequest | Self::PingResponse |
            Self::DeviceInfoRequest | Self::DeviceInfoResponse |
            Self::GetTimeRequest | Self::GetTimeResponse => false,

            _ => true,
        }
    }

    /// Check if the message needs authentication
    pub fn needs_authentication(&self) -> bool {
        match self {
            Self::HelloRequest | Self::HelloResponse |
            Self::ConnectRequest | Self::ConnectResponse |
            Self::DisconnectRequest | Self::DisconnectResponse |
            Self::PingRequest | Self::PingResponse |
            Self::DeviceInfoRequest | Self::DeviceInfoResponse |
            Self::GetTimeRequest | Self::GetTimeResponse => false,

            _ => true,
        }
    }

    /// Get the corresponding response type for a request type
    pub fn corresponding_response(&self) -> Option<Self> {
        match self {
            Self::HelloRequest => Some(Self::HelloResponse),
            Self::ConnectRequest => Some(Self::ConnectResponse),
            Self::DisconnectRequest => Some(Self::DisconnectResponse),
            Self::PingRequest => Some(Self::PingResponse),
            Self::DeviceInfoRequest => Some(Self::DeviceInfoResponse),
            Self::ListEntitiesRequest => Some(Self::ListEntitiesDoneResponse), // Special case
            Self::SubscribeStatesRequest => None, // Multiple responses
            Self::SubscribeLogsRequest => Some(Self::SubscribeLogsResponse),
            Self::CoverCommandRequest => None, // State updates only
            Self::FanCommandRequest => None, // State updates only
            Self::LightCommandRequest => None, // State updates only
            Self::SwitchCommandRequest => None, // State updates only
            Self::SubscribeHomeassistantServicesRequest => Some(Self::HomeassistantServiceResponse),
            Self::GetTimeRequest => Some(Self::GetTimeResponse),
            Self::SubscribeHomeAssistantStatesRequest => Some(Self::SubscribeHomeAssistantStateResponse),
            Self::ExecuteServiceRequest => None, // No direct response
            Self::CameraImageRequest => Some(Self::CameraImageResponse),
            Self::ClimateCommandRequest => None, // State updates only
            Self::NumberCommandRequest => None, // State updates only
            Self::SelectCommandRequest => None, // State updates only
            Self::SirenCommandRequest => None, // State updates only
            Self::LockCommandRequest => None, // State updates only
            Self::ButtonCommandRequest => None, // No direct response
            Self::MediaPlayerCommandRequest => None, // State updates only
            Self::SubscribeBluetoothLEAdvertisementsRequest => Some(Self::BluetoothLEAdvertisementResponse),
            Self::BluetoothDeviceRequest => Some(Self::BluetoothDeviceConnectionResponse), // Depends on request type
            Self::BluetoothGATTGetServicesRequest => Some(Self::BluetoothGATTGetServicesResponse),
            Self::BluetoothGATTReadRequest => Some(Self::BluetoothGATTReadResponse),
            Self::BluetoothGATTWriteRequest => Some(Self::BluetoothGATTWriteResponse),
            Self::BluetoothGATTReadDescriptorRequest => Some(Self::BluetoothGATTReadResponse), // Reused
            Self::BluetoothGATTWriteDescriptorRequest => Some(Self::BluetoothGATTWriteResponse), // Reused
            Self::BluetoothGATTNotifyRequest => Some(Self::BluetoothGATTNotifyResponse),
            Self::SubscribeBluetoothConnectionsFreeRequest => Some(Self::BluetoothConnectionsFreeResponse),
            Self::SubscribeVoiceAssistantRequest => None, // Multiple responses
            Self::VoiceAssistantAnnounceRequest => Some(Self::VoiceAssistantAnnounceFinished),
            Self::VoiceAssistantConfigurationRequest => Some(Self::VoiceAssistantConfigurationResponse),
            Self::VoiceAssistantSetConfiguration => None, // No direct response
            Self::UnsubscribeBluetoothLEAdvertisementsRequest => None, // No direct response
            Self::AlarmControlPanelCommandRequest => None, // State updates only
            Self::TextCommandRequest => None, // State updates only
            Self::DateCommandRequest => None, // State updates only
            Self::TimeCommandRequest => None, // State updates only
            Self::ValveCommandRequest => None, // State updates only
            Self::DateTimeCommandRequest => None, // State updates only
            Self::UpdateCommandRequest => None, // State updates only
            Self::NoiseEncryptionSetKeyRequest => Some(Self::NoiseEncryptionSetKeyResponse),
            Self::BluetoothScannerSetModeRequest => Some(Self::BluetoothScannerStateResponse),
            // Responses don't have corresponding responses
            _ => None,
        }
    }

    /// Check if the message is a request (as opposed to a response)
    pub fn is_request(&self) -> bool {
        match self {
            Self::HelloRequest | Self::ConnectRequest | Self::DisconnectRequest |
            Self::PingRequest | Self::DeviceInfoRequest | Self::ListEntitiesRequest |
            Self::SubscribeStatesRequest | Self::SubscribeLogsRequest |
            Self::CoverCommandRequest | Self::FanCommandRequest |
            Self::LightCommandRequest | Self::SwitchCommandRequest |
            Self::SubscribeHomeassistantServicesRequest | Self::GetTimeRequest |
            Self::SubscribeHomeAssistantStatesRequest | Self::ExecuteServiceRequest |
            Self::CameraImageRequest | Self::ClimateCommandRequest |
            Self::NumberCommandRequest | Self::SelectCommandRequest |
            Self::SirenCommandRequest | Self::LockCommandRequest |
            Self::ButtonCommandRequest | Self::MediaPlayerCommandRequest |
            Self::SubscribeBluetoothLEAdvertisementsRequest | Self::BluetoothDeviceRequest |
            Self::BluetoothGATTGetServicesRequest | Self::BluetoothGATTReadRequest |
            Self::BluetoothGATTWriteRequest | Self::BluetoothGATTReadDescriptorRequest |
            Self::BluetoothGATTWriteDescriptorRequest | Self::BluetoothGATTNotifyRequest |
            Self::SubscribeBluetoothConnectionsFreeRequest |
            Self::UnsubscribeBluetoothLEAdvertisementsRequest |
            Self::SubscribeVoiceAssistantRequest | Self::VoiceAssistantAnnounceRequest |
            Self::VoiceAssistantConfigurationRequest | Self::VoiceAssistantSetConfiguration |
            Self::AlarmControlPanelCommandRequest | Self::TextCommandRequest |
            Self::DateCommandRequest | Self::TimeCommandRequest |
            Self::ValveCommandRequest | Self::DateTimeCommandRequest |
            Self::UpdateCommandRequest | Self::NoiseEncryptionSetKeyRequest |
            Self::BluetoothScannerSetModeRequest => true,

            _ => false,
        }
    }
}