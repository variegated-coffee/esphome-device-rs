use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use crate::api::{DateCommandRequest, DateStateResponse, EntityCategory, ListEntitiesDateResponse};

#[derive(Clone, PartialEq)]
pub struct DateCommandData {
    pub key: u32,
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl<'a> From<DateCommandRequest<'a>> for DateCommandData {
    fn from(request: DateCommandRequest<'a>) -> Self {
        DateCommandData {
            key: request.key,
            year: request.year,
            month: request.month,
            day: request.day,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DateConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
}

impl<'a> Into<ListEntitiesDateResponse<'a>> for &DateConfig<'a> {
    fn into(self) -> ListEntitiesDateResponse<'a> {
        ListEntitiesDateResponse {
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
pub struct DateState {
    pub key: u32,
    pub missing_state: bool,
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl<'a> Into<DateStateResponse<'a>> for DateState {
    fn into(self) -> DateStateResponse<'a> {
        DateStateResponse {
            key: self.key,
            missing_state: self.missing_state,
            year: self.year,
            month: self.month,
            day: self.day,
            unknown_fields: UnknownFields::default(),
        }
    }
}