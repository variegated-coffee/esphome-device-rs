use femtopb::EnumValue::{Known, Unknown};
use femtopb::UnknownFields;
use crate::api::{CoverCommandRequest, CoverOperation, CoverStateResponse, EntityCategory, LegacyCoverCommand, LegacyCoverState, ListEntitiesCoverResponse};

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

#[derive(Clone, PartialEq)]
pub struct CoverCommandData {
    pub key: u32,
    pub position: Option<f32>,
    pub tilt: Option<f32>,
    pub stop: bool,
}

impl<'a> From<CoverCommandRequest<'a>> for CoverCommandData {
    fn from(request: CoverCommandRequest<'a>) -> Self {
        let (position, stop) = if request.has_position {
            (Some(request.position), request.stop)
        } else if request.has_legacy_command {
            match request.legacy_command {
                Known(LegacyCoverCommand::Open) => (Some(100.0), false),
                Known(LegacyCoverCommand::Close) => (Some(0.0), false),
                Known(LegacyCoverCommand::Stop) => (None, true),
                Unknown(_) => (None, request.stop)
            }
        } else {
            (None, request.stop)
        };
        
        CoverCommandData {
            key: request.key,
            position,
            tilt: if request.has_tilt { Some(request.tilt) } else { None },
            stop,
        }
    }
}

pub struct CoverConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub assumed_state: bool,
    pub supports_position: bool,
    pub supports_tilt: bool,
    pub device_class: &'a str,
    pub disabled_by_default: bool,
    pub icon: &'a str,
    pub entity_category: EntityCategory,
    pub supports_stop: bool,
}

impl<'a> Into<ListEntitiesCoverResponse<'a>> for &CoverConfig<'a> {
    fn into(self) -> ListEntitiesCoverResponse<'a> {
        ListEntitiesCoverResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            assumed_state: self.assumed_state,
            supports_position: self.supports_position,
            supports_tilt: self.supports_tilt,
            device_class: self.device_class,
            disabled_by_default: self.disabled_by_default,
            icon: self.icon,
            entity_category: Known(self.entity_category),
            supports_stop: self.supports_stop,
            unknown_fields: UnknownFields::default(),
        }
    }
}