use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use crate::api::{EntityCategory, ListEntitiesValveResponse, ValveCommandRequest, ValveOperation, ValveStateResponse};

#[derive(Clone, PartialEq)]
pub struct ValveCommandData {
    pub key: u32,
    pub position: Option<f32>,
    pub stop: bool,
}

impl<'a> From<ValveCommandRequest<'a>> for ValveCommandData {
    fn from(request: ValveCommandRequest<'a>) -> Self {
        ValveCommandData {
            key: request.key,
            position: if request.has_position { Some(request.position) } else { None },
            stop: request.stop,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ValveConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
    pub device_class: &'a str,
    pub assumed_state: bool,
    pub supports_position: bool,
    pub supports_stop: bool,
}

impl<'a> Into<ListEntitiesValveResponse<'a>> for &ValveConfig<'a> {
    fn into(self) -> ListEntitiesValveResponse<'a> {
        ListEntitiesValveResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            icon: self.icon,
            disabled_by_default: self.disabled_by_default,
            entity_category: Known(self.entity_category),
            device_class: self.device_class,
            assumed_state: self.assumed_state,
            supports_position: self.supports_position,
            supports_stop: self.supports_stop,
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct ValveState {
    pub key: u32,
    pub position: f32,
    pub current_operation: ValveOperation,
}

impl<'a> Into<ValveStateResponse<'a>> for ValveState {
    fn into(self) -> ValveStateResponse<'a> {
        ValveStateResponse {
            key: self.key,
            position: self.position,
            current_operation: Known(self.current_operation),
            unknown_fields: UnknownFields::default(),
        }
    }
}