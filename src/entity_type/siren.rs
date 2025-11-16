use femtopb::EnumValue::Known;
use femtopb::Repeated;
use crate::api::{EntityCategory, ListEntitiesSirenResponse, SirenCommandRequest, SirenStateResponse};

#[derive(Clone, PartialEq)]
pub struct SirenCommandData {
    pub key: u32,
    pub state: Option<bool>,
    pub tone: Option<String>,
    pub duration: Option<u32>,
    pub volume: Option<f32>,
}

impl<'a> From<SirenCommandRequest<'a>> for SirenCommandData {
    fn from(request: SirenCommandRequest<'a>) -> Self {
        SirenCommandData {
            key: request.key,
            state: if request.has_state { Some(request.state) } else { None },
            tone: if request.has_tone {
                String::try_from(request.tone).ok()
            } else { None },
            duration: if request.has_duration { Some(request.duration) } else { None },
            volume: if request.has_volume { Some(request.volume) } else { None },
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SirenConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub disabled_by_default: bool,
    pub tones: &'a [&'a str],
    pub supports_duration: bool,
    pub supports_volume: bool,
    pub entity_category: EntityCategory,
}

impl<'a> Into<ListEntitiesSirenResponse<'a>> for &SirenConfig<'a> {
    fn into(self) -> ListEntitiesSirenResponse<'a> {
        ListEntitiesSirenResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            icon: self.icon,
            disabled_by_default: self.disabled_by_default,
            tones: Repeated::from(self.tones),
            supports_duration: self.supports_duration,
            supports_volume: self.supports_volume,
            entity_category: Known(self.entity_category),
            unknown_fields: femtopb::UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct SirenState {
    pub key: u32,
    pub state: bool,
}

impl<'a> Into<SirenStateResponse<'a>> for SirenState {
    fn into(self) -> SirenStateResponse<'a> {
        SirenStateResponse {
            key: self.key,
            state: self.state,
            unknown_fields: femtopb::UnknownFields::default(),
        }
    }
}