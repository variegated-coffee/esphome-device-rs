extern crate alloc;

use crate::entity_type::alarm_control_panel::{AlarmControlPanelCommandData, AlarmControlPanelConfig, AlarmControlPanelEntityState};
pub use crate::entity_type::binary_sensor::{BinarySensorConfig, BinarySensorState};
use crate::entity_type::button::{ButtonCommandData, ButtonConfig};
use crate::entity_type::climate::{ClimateCommandData, ClimateConfig, ClimateState};
use crate::entity_type::cover::{CoverCommandData, CoverConfig, CoverState};
use crate::entity_type::date::{DateCommandData, DateConfig, DateState};
use crate::entity_type::datetime::{DateTimeCommandData, DateTimeConfig, DateTimeState};
use crate::entity_type::event::{EventConfig, EventState};
use crate::entity_type::fan::{FanCommandData, FanConfig, FanState};
use crate::entity_type::light::{LightCommandData, LightConfig, LightState};
use crate::entity_type::lock::{LockCommandData, LockConfig, LockEntityState};
use crate::entity_type::number::{NumberCommandData, NumberConfig, NumberState};
use crate::entity_type::select::{SelectCommandData, SelectConfig, SelectState};
use crate::entity_type::sensor::{SensorConfig, SensorState};
use crate::entity_type::siren::{SirenCommandData, SirenConfig, SirenState};
use crate::entity_type::switch::{SwitchCommandData, SwitchConfig, SwitchState};
use crate::entity_type::text::{TextCommandData, TextConfig, TextState};
use crate::entity_type::text_sensor::{TextSensorConfig, TextSensorState};
use crate::entity_type::time::{TimeCommandData, TimeConfig, TimeState};
use crate::entity_type::valve::{ValveCommandData, ValveConfig, ValveState};

pub mod api;
pub mod metadata;
#[cfg(feature = "embassy_net")]
pub mod embassy_net;
#[cfg(feature = "std")]
pub mod std;
pub mod server;
pub mod entity_type;

#[derive(Default, Debug)]
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


#[derive(Clone, Copy, PartialEq)]
pub enum StateChange<'a> {
    BinarySensorChange(BinarySensorState),
    SensorStateChange(SensorState),
    TextSensorStateChange(TextSensorState<'a>),
    CoverChange(CoverState),
    FanStateChange(FanState<'a>),
    LightStateChange(LightState<'a>),
    SwitchStateChange(SwitchState),
    ClimateStateChange(ClimateState<'a>),
    NumberStateChange(NumberState),
    SelectStateChange(SelectState<'a>),
    SirenStateChange(SirenState),
    LockStateChange(LockEntityState),
    AlarmControlPanelStateChange(AlarmControlPanelEntityState),
    TextStateChange(TextState<'a>),
    DateStateChange(DateState),
    TimeStateChange(TimeState),
    EventStateChange(EventState<'a>),
    ValveStateChange(ValveState),
    DateTimeStateChange(DateTimeState),
}

pub enum ClientEvent {
    Connected,
    Disconnected,
    CommandReceived(Command),
    SubscribedToStates,
    SubscribedToLogs,
    ReceivedTime(u32) // Timestamp in seconds since epoch
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

#[derive(Clone, PartialEq, Debug)]
pub enum EntityConfig<'a> {
    BinarySensor(BinarySensorConfig<'a>),
    Sensor(SensorConfig<'a>),
    TextSensor(TextSensorConfig<'a>),
    Cover(CoverConfig<'a>),
    Fan(FanConfig<'a>),
    Light(LightConfig<'a>),
    Switch(SwitchConfig<'a>),
    Climate(ClimateConfig<'a>),
    Number(NumberConfig<'a>),
    Select(SelectConfig<'a>),
    Siren(SirenConfig<'a>),
    Lock(LockConfig<'a>),
    Button(ButtonConfig<'a>),
    AlarmControlPanel(AlarmControlPanelConfig<'a>),
    Text(TextConfig<'a>),
    Date(DateConfig<'a>),
    Time(TimeConfig<'a>),
    Event(EventConfig<'a>),
    Valve(ValveConfig<'a>),
    DateTime(DateTimeConfig<'a>),
}
