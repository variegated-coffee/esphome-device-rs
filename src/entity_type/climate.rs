use femtopb::EnumValue::Known;
use femtopb::{EnumValue, Packed, Repeated, UnknownFields};
use crate::api::{ClimateAction, ClimateCommandRequest, ClimateFanMode, ClimateMode, ClimatePreset, ClimateStateResponse, ClimateSwingMode, EntityCategory, ListEntitiesClimateResponse};

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

impl<'a> From<ClimateCommandRequest<'a>> for ClimateCommandData {
    fn from(request: ClimateCommandRequest<'a>) -> Self {
        ClimateCommandData {
            key: request.key,
            mode: if request.has_mode {
                match request.mode {
                    Known(m) => Some(m),
                    _ => None,
                }
            } else { None },
            target_temperature: if request.has_target_temperature { Some(request.target_temperature) } else { None },
            target_temperature_low: if request.has_target_temperature_low { Some(request.target_temperature_low) } else { None },
            target_temperature_high: if request.has_target_temperature_high { Some(request.target_temperature_high) } else { None },
            fan_mode: if request.has_fan_mode {
                match request.fan_mode {
                    Known(f) => Some(f),
                    _ => None,
                }
            } else { None },
            swing_mode: if request.has_swing_mode {
                match request.swing_mode {
                    Known(s) => Some(s),
                    _ => None,
                }
            } else { None },
            custom_fan_mode: if request.has_custom_fan_mode {
                String::try_from(request.custom_fan_mode).ok()
            } else { None },
            preset: if request.has_preset {
                match request.preset {
                    Known(p) => Some(p),
                    _ => None,
                }
            } else { None },
            custom_preset: if request.has_custom_preset {
                String::try_from(request.custom_preset).ok()
            } else { None },
            target_humidity: if request.has_target_humidity {
                Some(request.target_humidity)
            } else { None },
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ClimateConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub supports_current_temperature: bool,
    pub supports_two_point_target_temperatures: bool,
    pub supported_modes: &'a [EnumValue<ClimateMode>],
    pub visual_min_temperature: f32,
    pub visual_max_temperature: f32,
    pub visual_target_temperature_step: f32,
    pub supports_action: bool,
    pub supported_fan_modes: &'a [EnumValue<ClimateFanMode>],
    pub supported_swing_modes: &'a [EnumValue<ClimateSwingMode>],
    pub supported_custom_fan_modes: &'a [&'a str],
    pub supported_presets: &'a [EnumValue<ClimatePreset>],
    pub supported_custom_presets: &'a [&'a str],
    pub disabled_by_default: bool,
    pub icon: &'a str,
    pub entity_category: EntityCategory,
    pub visual_current_temperature_step: f32,
    pub supports_current_humidity: bool,
    pub supports_target_humidity: bool,
    pub visual_min_humidity: f32,
    pub visual_max_humidity: f32,
}

impl<'a> Into<ListEntitiesClimateResponse<'a>> for &ClimateConfig<'a> {
    fn into(self) -> ListEntitiesClimateResponse<'a> {
        let legacy_supports_away = self.supported_presets.contains(&Known(ClimatePreset::Away));
        
        ListEntitiesClimateResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            supports_current_temperature: self.supports_current_temperature,
            supports_two_point_target_temperature: self.supports_two_point_target_temperatures,
            supported_modes: Packed::from(self.supported_modes),
            visual_min_temperature: self.visual_min_temperature,
            visual_max_temperature: self.visual_max_temperature,
            visual_target_temperature_step: self.visual_target_temperature_step,
            supports_action: self.supports_action,
            supported_fan_modes: Packed::from(self.supported_fan_modes),
            supported_swing_modes: Packed::from(self.supported_swing_modes),
            supported_custom_fan_modes: Repeated::from(self.supported_custom_fan_modes),
            supported_presets: Packed::from(self.supported_presets),
            supported_custom_presets: Repeated::from(self.supported_custom_presets),
            disabled_by_default: self.disabled_by_default,
            icon: self.icon,
            entity_category: Known(self.entity_category),
            visual_current_temperature_step: self.visual_current_temperature_step,
            supports_current_humidity: self.supports_current_humidity,
            visual_min_humidity: self.visual_min_humidity,
            visual_max_humidity: self.visual_max_humidity,
            legacy_supports_away,
            supports_target_humidity: self.supports_target_humidity,
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct ClimateState<'a> {
    pub key: u32,
    pub mode: ClimateMode,
    pub current_temperature: f32,
    pub target_temperature: f32,
    pub target_temperature_low: f32,
    pub target_temperature_high: f32,
    pub action: ClimateAction,
    pub fan_mode: ClimateFanMode,
    pub swing_mode: ClimateSwingMode,
    pub custom_fan_mode: &'a str,
    pub preset: ClimatePreset,
    pub custom_preset: &'a str,
    pub current_humidity: f32,
    pub target_humidity: f32,
}

impl<'a> Into<ClimateStateResponse<'a>> for ClimateState<'a> {
    fn into(self) -> ClimateStateResponse<'a> {
        ClimateStateResponse {
            key: self.key,
            mode: Known(self.mode),
            current_temperature: self.current_temperature,
            target_temperature: self.target_temperature,
            target_temperature_low: self.target_temperature_low,
            target_temperature_high: self.target_temperature_high,
            action: Known(self.action),
            fan_mode: Known(self.fan_mode),
            swing_mode: Known(self.swing_mode),
            custom_fan_mode: self.custom_fan_mode,
            preset: Known(self.preset),
            custom_preset: self.custom_preset,
            current_humidity: self.current_humidity,
            target_humidity: self.target_humidity,
            unused_legacy_away: false,
            unknown_fields: UnknownFields::default(),
        }
    }
}