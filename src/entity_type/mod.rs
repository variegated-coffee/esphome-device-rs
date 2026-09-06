//! One module per ESPHome entity type, each behind its own feature.
//!
//! The modules do not reference each other -- every one imports only `crate::api`, `femtopb`
//! and `alloc` -- which is what makes them separable at all. See the `[features]` block in
//! `Cargo.toml` for why a device would want to compile only some of them.

#[cfg(feature = "alarm_control_panel")]
pub mod alarm_control_panel;
#[cfg(feature = "binary_sensor")]
pub mod binary_sensor;
#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "climate")]
pub mod climate;
#[cfg(feature = "cover")]
pub mod cover;
#[cfg(feature = "date")]
pub mod date;
#[cfg(feature = "datetime")]
pub mod datetime;
#[cfg(feature = "event")]
pub mod event;
#[cfg(feature = "fan")]
pub mod fan;
#[cfg(feature = "light")]
pub mod light;
#[cfg(feature = "lock")]
pub mod lock;
#[cfg(feature = "number")]
pub mod number;
#[cfg(feature = "select")]
pub mod select;
#[cfg(feature = "sensor")]
pub mod sensor;
#[cfg(feature = "siren")]
pub mod siren;
#[cfg(feature = "switch")]
pub mod switch;
#[cfg(feature = "text")]
pub mod text;
#[cfg(feature = "text_sensor")]
pub mod text_sensor;
#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "valve")]
pub mod valve;
