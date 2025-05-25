use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use crate::api::{EntityCategory, ListEntitiesTimeResponse, TimeCommandRequest, TimeStateResponse};

#[derive(Clone, PartialEq)]
pub struct TimeCommandData {
    pub key: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl<'a> From<TimeCommandRequest<'a>> for TimeCommandData {
    fn from(request: TimeCommandRequest<'a>) -> Self {
        TimeCommandData {
            key: request.key,
            hour: request.hour,
            minute: request.minute,
            second: request.second,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct TimeConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
}

impl<'a> Into<ListEntitiesTimeResponse<'a>> for &TimeConfig<'a> {
    fn into(self) -> ListEntitiesTimeResponse<'a> {
        ListEntitiesTimeResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            icon: self.icon,
            disabled_by_default: self.disabled_by_default,
            entity_category: Known(self.entity_category),
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct TimeState {
    pub key: u32,
    pub missing_state: bool,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl<'a> Into<TimeStateResponse<'a>> for TimeState {
    fn into(self) -> TimeStateResponse<'a> {
        TimeStateResponse {
            key: self.key,
            missing_state: self.missing_state,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
            unknown_fields: UnknownFields::default(),
        }
    }
}