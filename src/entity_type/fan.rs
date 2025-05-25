use femtopb::EnumValue::{Known, Unknown};
use femtopb::{Repeated, UnknownFields};
use crate::api::{EntityCategory, FanCommandRequest, FanDirection, FanSpeed, FanStateResponse, ListEntitiesFanResponse};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FanState<'a> {
    pub key: u32,
    pub state: bool,
    pub oscillating: bool,
    pub speed_level: i32,
    pub direction: FanDirection,
    pub preset_mode: &'a str,
}

impl<'a> Into<FanStateResponse<'a>> for FanState<'a> {
    fn into(self) -> FanStateResponse<'a> {
        let speed = if self.speed_level < 33 {
            FanSpeed::Low
        } else if self.speed_level < 66 {
            FanSpeed::Medium
        } else {
            FanSpeed::High
        };

        FanStateResponse {
            key: self.key,
            state: self.state,
            oscillating: self.oscillating,
            speed: Known(speed),
            speed_level: self.speed_level,
            direction: Known(self.direction),
            preset_mode: self.preset_mode,
            unknown_fields: UnknownFields::default(),
        }
    }
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

impl<'a> From<FanCommandRequest<'a>> for FanCommandData {
    fn from(request: FanCommandRequest<'a>) -> Self {
        let speed_level = if request.has_speed_level {
            Some(request.speed_level)
        } else if request.has_speed {
            match request.speed {
                Known(FanSpeed::Low) => Some(0),
                Known(FanSpeed::Medium) => Some(50),
                Known(FanSpeed::High) => Some(100),
                Unknown(_) => None, // Handle unknown speed gracefully
            }
        } else { None };

        let state = if request.has_state { Some(request.state) } else { None };
        let oscillating = if request.has_oscillating { Some(request.oscillating) } else { None };
        let direction = if request.has_direction { 
            match request.direction {
                Known(FanDirection::Forward) => Some(FanDirection::Forward),
                Known(FanDirection::Reverse) => Some(FanDirection::Reverse),
                Unknown(_) => None, // Handle unknown direction gracefully
            }
        } else { 
            None 
        };
        let preset_mode = if request.has_preset_mode { Some(request.preset_mode.to_string()) } else { None };

        FanCommandData {
            key: request.key,
            state,
            oscillating,
            direction,
            speed_level,
            preset_mode,
        }
    }
}

pub struct FanConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub supports_oscillation: bool,
    pub supports_speed: bool,
    pub supports_direction: bool,
    pub supported_speed_count: i32,
    pub disabled_by_default: bool,
    pub icon: &'a str,
    pub entity_category: EntityCategory,
    pub supported_preset_modes: &'a [&'a str],
}

impl<'a> Into<ListEntitiesFanResponse<'a>> for &FanConfig<'a> {
    fn into(self) -> ListEntitiesFanResponse<'a> {
        ListEntitiesFanResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            supports_oscillation: self.supports_oscillation,
            supports_speed: self.supports_speed,
            supports_direction: self.supports_direction,
            supported_speed_count: self.supported_speed_count,
            disabled_by_default: self.disabled_by_default,
            icon: self.icon,
            entity_category: Known(self.entity_category),
            supported_preset_modes: Repeated::from_slice(self.supported_preset_modes),
            unknown_fields: UnknownFields::default(),
        }
    }
}