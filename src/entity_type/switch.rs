use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use crate::api::{EntityCategory, ListEntitiesSwitchResponse, SwitchCommandRequest, SwitchStateResponse};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct SwitchState {
    pub key: u32,
    pub state: bool,
}

impl<'a> Into<SwitchStateResponse<'a>> for SwitchState {
    fn into(self) -> SwitchStateResponse<'a> {
        SwitchStateResponse {
            key: self.key,
            state: self.state,
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct SwitchCommandData {
    pub key: u32,
    pub state: bool,
}

impl From<SwitchCommandRequest<'_>> for SwitchCommandData {
    fn from(request: SwitchCommandRequest) -> Self {
        SwitchCommandData {
            key: request.key,
            state: request.state,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SwitchConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub assumed_state: bool,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
    pub device_class: &'a str,
}

impl<'a> Into<ListEntitiesSwitchResponse<'a>> for &SwitchConfig<'a> {
    fn into(self) -> ListEntitiesSwitchResponse<'a> {
        ListEntitiesSwitchResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            icon: self.icon,
            assumed_state: self.assumed_state,
            disabled_by_default: self.disabled_by_default,
            entity_category: Known(self.entity_category),
            device_class: self.device_class,
            unknown_fields: UnknownFields::default(),
        }
    }
}
