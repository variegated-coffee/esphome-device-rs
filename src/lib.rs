// no_std only when the `std` backend is off. Going unconditionally `#![no_std]`
// and adding `extern crate std` for the feature does not work here: this crate
// already has a module named `std` (the async-std backend), and the extern
// crate declaration collides with it at the crate root.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(feature = "alarm_control_panel")]
use crate::entity_type::alarm_control_panel::{AlarmControlPanelCommandData, AlarmControlPanelConfig, AlarmControlPanelEntityState};
#[cfg(feature = "binary_sensor")]
pub use crate::entity_type::binary_sensor::{BinarySensorConfig, BinarySensorState};
#[cfg(feature = "button")]
use crate::entity_type::button::{ButtonCommandData, ButtonConfig};
#[cfg(feature = "climate")]
use crate::entity_type::climate::{ClimateCommandData, ClimateConfig, ClimateState};
#[cfg(feature = "cover")]
use crate::entity_type::cover::{CoverCommandData, CoverConfig, CoverState};
#[cfg(feature = "date")]
use crate::entity_type::date::{DateCommandData, DateConfig, DateState};
#[cfg(feature = "datetime")]
use crate::entity_type::datetime::{DateTimeCommandData, DateTimeConfig, DateTimeState};
#[cfg(feature = "event")]
use crate::entity_type::event::{EventConfig, EventState};
#[cfg(feature = "fan")]
use crate::entity_type::fan::{FanCommandData, FanConfig, FanState};
#[cfg(feature = "light")]
use crate::entity_type::light::{LightCommandData, LightConfig, LightState};
#[cfg(feature = "lock")]
use crate::entity_type::lock::{LockCommandData, LockConfig, LockEntityState};
#[cfg(feature = "number")]
use crate::entity_type::number::{NumberCommandData, NumberConfig, NumberState};
#[cfg(feature = "select")]
use crate::entity_type::select::{SelectCommandData, SelectConfig, SelectState};
#[cfg(feature = "sensor")]
use crate::entity_type::sensor::{SensorConfig, SensorState};
#[cfg(feature = "siren")]
use crate::entity_type::siren::{SirenCommandData, SirenConfig, SirenState};
#[cfg(feature = "switch")]
use crate::entity_type::switch::{SwitchCommandData, SwitchConfig, SwitchState};
#[cfg(feature = "text")]
use crate::entity_type::text::{TextCommandData, TextConfig, TextState};
#[cfg(feature = "text_sensor")]
use crate::entity_type::text_sensor::{TextSensorConfig, TextSensorState};
#[cfg(feature = "time")]
use crate::entity_type::time::{TimeCommandData, TimeConfig, TimeState};
#[cfg(feature = "valve")]
use crate::entity_type::valve::{ValveCommandData, ValveConfig, ValveState};

pub mod api;
pub mod error;
pub mod metadata;
#[cfg(feature = "embassy_net")]
pub mod embassy_net;
#[cfg(feature = "std")]
pub mod std;
pub mod server;
pub mod entity_type;

pub use error::{EspHomeError, Result};

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


/// A state report for one entity, sized by its largest enabled variant.
///
/// **Which variants exist depends on the entity-type features.** That is the point: a consumer
/// stores these by value in a queue, so the enum's size is multiplied by the queue depth, and
/// with every type compiled in the size is set by `ClimateState` at 64 bytes -- against 12 to
/// 24 for the types a typical sensor device actually reports.
#[derive(Clone, Copy, PartialEq)]
pub enum StateChange<'a> {
    /// Ties `'a` to the enum regardless of which features are on. Not constructible.
    ///
    /// Only seven of the nineteen real variants carry `'a`, so a selection that excludes all
    /// of them -- `--features sensor` on its own, for instance -- would leave the parameter
    /// unconstrained and fail with `error[E0392]: parameter 'a is never used`, which says
    /// nothing about features and would be a miserable thing to debug.
    ///
    /// `Infallible` makes the variant uninhabited, so it occupies no space in the layout and,
    /// since `min_exhaustive_patterns` stabilised in Rust 1.82, needs no arm in any `match`.
    #[doc(hidden)]
    _Uninhabited(core::marker::PhantomData<&'a ()>, core::convert::Infallible),
    #[cfg(feature = "binary_sensor")]
    BinarySensorChange(BinarySensorState),
    #[cfg(feature = "sensor")]
    SensorStateChange(SensorState),
    #[cfg(feature = "text_sensor")]
    TextSensorStateChange(TextSensorState<'a>),
    #[cfg(feature = "cover")]
    CoverChange(CoverState),
    #[cfg(feature = "fan")]
    FanStateChange(FanState<'a>),
    #[cfg(feature = "light")]
    LightStateChange(LightState<'a>),
    #[cfg(feature = "switch")]
    SwitchStateChange(SwitchState),
    #[cfg(feature = "climate")]
    ClimateStateChange(ClimateState<'a>),
    #[cfg(feature = "number")]
    NumberStateChange(NumberState),
    #[cfg(feature = "select")]
    SelectStateChange(SelectState<'a>),
    #[cfg(feature = "siren")]
    SirenStateChange(SirenState),
    #[cfg(feature = "lock")]
    LockStateChange(LockEntityState),
    #[cfg(feature = "alarm_control_panel")]
    AlarmControlPanelStateChange(AlarmControlPanelEntityState),
    #[cfg(feature = "text")]
    TextStateChange(TextState<'a>),
    #[cfg(feature = "date")]
    DateStateChange(DateState),
    #[cfg(feature = "time")]
    TimeStateChange(TimeState),
    #[cfg(feature = "event")]
    EventStateChange(EventState<'a>),
    #[cfg(feature = "valve")]
    ValveStateChange(ValveState),
    #[cfg(feature = "datetime")]
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

/// A command from a client, for the entity types that accept one.
///
/// Sixteen of the twenty types are represented -- `sensor`, `binary_sensor`, `text_sensor` and
/// `event` report only. No lifetime parameter: every `CommandData` owns its strings, so unlike
/// [`StateChange`] and [`EntityConfig`] this needs no uninhabited variant.
#[derive(Clone, PartialEq)]
pub enum Command {
    #[cfg(feature = "cover")]
    CoverCommand(CoverCommandData),
    #[cfg(feature = "fan")]
    FanCommand(FanCommandData),
    #[cfg(feature = "light")]
    LightCommand(LightCommandData),
    #[cfg(feature = "switch")]
    SwitchCommand(SwitchCommandData),
    #[cfg(feature = "climate")]
    ClimateCommand(ClimateCommandData),
    #[cfg(feature = "number")]
    NumberCommand(NumberCommandData),
    #[cfg(feature = "select")]
    SelectCommand(SelectCommandData),
    #[cfg(feature = "siren")]
    SirenCommand(SirenCommandData),
    #[cfg(feature = "lock")]
    LockCommand(LockCommandData),
    #[cfg(feature = "button")]
    ButtonCommand(ButtonCommandData),
    #[cfg(feature = "alarm_control_panel")]
    AlarmControlPanelCommand(AlarmControlPanelCommandData),
    #[cfg(feature = "text")]
    TextCommand(TextCommandData),
    #[cfg(feature = "date")]
    DateCommand(DateCommandData),
    #[cfg(feature = "time")]
    TimeCommand(TimeCommandData),
    #[cfg(feature = "valve")]
    ValveCommand(ValveCommandData),
    #[cfg(feature = "datetime")]
    DateTimeCommand(DateTimeCommandData),
}

/// The declaration of one entity, sized by its largest enabled variant.
///
/// **Which variants exist depends on the entity-type features**, and this is where the choice
/// costs the most: a device holds a table of these by value, so the enum's size multiplies by
/// the entity count. With every type compiled in it is 120 bytes, set by `ClimateConfig`;
/// without `climate` the ceiling is `NumberConfig` at 76, giving 80.
#[derive(Clone, PartialEq, Debug)]
pub enum EntityConfig<'a> {
    /// Ties `'a` to the enum when no variant is enabled. Not constructible.
    ///
    /// Every real variant here carries `'a`, so unlike [`StateChange`] this only matters when
    /// *all* entity types are off -- but a crate whose zero-feature build fails with
    /// `error[E0392]` is not one anyone should have to diagnose. See the twin on
    /// [`StateChange`] for why `Infallible` costs nothing.
    #[doc(hidden)]
    _Uninhabited(core::marker::PhantomData<&'a ()>, core::convert::Infallible),
    #[cfg(feature = "binary_sensor")]
    BinarySensor(BinarySensorConfig<'a>),
    #[cfg(feature = "sensor")]
    Sensor(SensorConfig<'a>),
    #[cfg(feature = "text_sensor")]
    TextSensor(TextSensorConfig<'a>),
    #[cfg(feature = "cover")]
    Cover(CoverConfig<'a>),
    #[cfg(feature = "fan")]
    Fan(FanConfig<'a>),
    #[cfg(feature = "light")]
    Light(LightConfig<'a>),
    #[cfg(feature = "switch")]
    Switch(SwitchConfig<'a>),
    #[cfg(feature = "climate")]
    Climate(ClimateConfig<'a>),
    #[cfg(feature = "number")]
    Number(NumberConfig<'a>),
    #[cfg(feature = "select")]
    Select(SelectConfig<'a>),
    #[cfg(feature = "siren")]
    Siren(SirenConfig<'a>),
    #[cfg(feature = "lock")]
    Lock(LockConfig<'a>),
    #[cfg(feature = "button")]
    Button(ButtonConfig<'a>),
    #[cfg(feature = "alarm_control_panel")]
    AlarmControlPanel(AlarmControlPanelConfig<'a>),
    #[cfg(feature = "text")]
    Text(TextConfig<'a>),
    #[cfg(feature = "date")]
    Date(DateConfig<'a>),
    #[cfg(feature = "time")]
    Time(TimeConfig<'a>),
    #[cfg(feature = "event")]
    Event(EventConfig<'a>),
    #[cfg(feature = "valve")]
    Valve(ValveConfig<'a>),
    #[cfg(feature = "datetime")]
    DateTime(DateTimeConfig<'a>),
}
