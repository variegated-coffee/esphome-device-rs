use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use crate::api::{DateTimeCommandRequest, DateTimeStateResponse, EntityCategory, ListEntitiesDateTimeResponse};

#[derive(Clone, PartialEq)]
pub struct DateTimeCommandData {
    pub key: u32,
    pub epoch_seconds: u32,
}

impl<'a> From<DateTimeCommandRequest<'a>> for DateTimeCommandData {
    fn from(request: DateTimeCommandRequest<'a>) -> Self {
        DateTimeCommandData {
            key: request.key,
            epoch_seconds: request.epoch_seconds,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DateTimeConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
}

impl<'a> Into<ListEntitiesDateTimeResponse<'a>> for &DateTimeConfig<'a> {
    fn into(self) -> ListEntitiesDateTimeResponse<'a> {
        ListEntitiesDateTimeResponse {
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
pub struct DateTimeState {
    pub key: u32,
    pub missing_state: bool,
    pub epoch_seconds: u32,
}

impl<'a> Into<DateTimeStateResponse<'a>> for DateTimeState {
    fn into(self) -> DateTimeStateResponse<'a> {
        DateTimeStateResponse {
            key: self.key,
            missing_state: self.missing_state,
            epoch_seconds: self.epoch_seconds,
            unknown_fields: UnknownFields::default(),
        }
    }
}