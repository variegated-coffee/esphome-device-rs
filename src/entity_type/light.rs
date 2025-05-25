use femtopb::EnumValue::Known;
use femtopb::{EnumValue, Packed, Repeated, UnknownFields};
use crate::api::{ColorMode, EntityCategory, LightCommandRequest, LightStateResponse, ListEntitiesLightResponse};
use crate::api::ColorMode::Unknown;

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

impl<'a> Into<LightStateResponse<'a>> for LightState<'a> {
    fn into(self) -> LightStateResponse<'a> {
        LightStateResponse {
            key: self.key,
            state: self.state,
            brightness: self.brightness,
            color_mode: Known(self.color_mode),
            color_brightness: self.color_brightness,
            red: self.rgb.red,
            green: self.rgb.green,
            blue: self.rgb.blue,
            white: self.white,
            color_temperature: self.color_temperature,
            cold_white: self.cold_white,
            warm_white: self.warm_white,
            effect: self.effect,
            unknown_fields: UnknownFields::default(),
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

impl<'a> From<LightCommandRequest<'a>> for LightCommandData {
    fn from(request: LightCommandRequest<'a>) -> Self {
        let rgb = if request.has_rgb {
            Some(RgbColor {
                red: request.red,
                green: request.green,
                blue: request.blue,
            })
        } else {
            None
        };

        LightCommandData {
            key: request.key,
            state: if request.has_state { Some(request.state) } else { None },
            brightness: if request.has_brightness { Some(request.brightness) } else { None },
            color_mode: if request.has_color_mode {
                match request.color_mode {
                    Known(mode) => Some(mode),
                    _ => None,
                }
            } else { 
                None 
            },
            color_brightness: if request.has_color_brightness { Some(request.color_brightness) } else { None },
            rgb,
            white: if request.has_white { Some(request.white) } else { None },
            color_temperature: if request.has_color_temperature { Some(request.color_temperature) } else { None },
            cold_white: if request.has_cold_white { Some(request.cold_white) } else { None },
            warm_white: if request.has_warm_white { Some(request.warm_white) } else { None },
            transition_length: if request.has_transition_length { Some(request.transition_length) } else { None },
            flash_length: if request.has_flash_length { Some(request.flash_length) } else { None },
            effect: if request.has_effect { Some(request.effect.to_string()) } else { None },
        }
    }
}

pub struct LightConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub supported_color_modes: &'a [EnumValue<ColorMode>],
    pub min_mireds: f32,
    pub max_mireds: f32,
    pub effects: &'a [&'a str],
    pub disabled_by_default: bool,
    pub icon: &'a str,
    pub entity_category: EntityCategory,
}

impl<'a> Into<ListEntitiesLightResponse<'a>> for &LightConfig<'a> {
    fn into(self) -> ListEntitiesLightResponse<'a> {
        let legacy_supports_brightness = self.supported_color_modes.contains(&Known(ColorMode::Brightness));
        let legacy_supports_rgb = self.supported_color_modes.contains(&Known(ColorMode::Rgb));
        let legacy_supports_color_temperature = self.supported_color_modes.contains(&Known(ColorMode::ColorTemperature));
        let legacy_supports_white_value = self.supported_color_modes.contains(&Known(ColorMode::White));
        
        ListEntitiesLightResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            supported_color_modes: Packed::from(self.supported_color_modes),
            min_mireds: self.min_mireds,
            max_mireds: self.max_mireds,
            effects: Repeated::from(self.effects),
            disabled_by_default: self.disabled_by_default,
            icon: self.icon,
            entity_category: Known(self.entity_category),
            legacy_supports_brightness,
            legacy_supports_rgb,
            legacy_supports_color_temperature,
            legacy_supports_white_value,
            unknown_fields: UnknownFields::default(),
        }
    }
}