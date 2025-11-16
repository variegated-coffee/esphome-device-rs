use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use crate::api::{EntityCategory, ListEntitiesNumberResponse, NumberCommandRequest, NumberMode, NumberStateResponse};

#[derive(Clone, PartialEq)]
pub struct NumberCommandData {
    pub key: u32,
    pub state: f32,
}

impl<'a> From<NumberCommandRequest<'a>> for NumberCommandData {
    fn from(request: NumberCommandRequest<'a>) -> Self {
        NumberCommandData {
            key: request.key,
            state: request.state,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct NumberConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub min_value: f32,
    pub max_value: f32,
    pub step: f32,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
    pub unit_of_measurement: &'a str,
    pub mode: NumberMode,
    pub device_class: &'a str,
}

impl<'a> Into<ListEntitiesNumberResponse<'a>> for &NumberConfig<'a> {
    fn into(self) -> ListEntitiesNumberResponse<'a> {
        ListEntitiesNumberResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            icon: self.icon,
            min_value: self.min_value,
            max_value: self.max_value,
            step: self.step,
            disabled_by_default: self.disabled_by_default,
            entity_category: Known(self.entity_category),
            unit_of_measurement: self.unit_of_measurement,
            mode: Known(self.mode),
            device_class: self.device_class,
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct NumberState {
    pub key: u32,
    pub state: f32,
    pub missing_state: bool,
}

impl<'a> Into<NumberStateResponse<'a>> for NumberState {
    fn into(self) -> NumberStateResponse<'a> {
        NumberStateResponse {
            key: self.key,
            state: self.state,
            missing_state: self.missing_state,
            unknown_fields: UnknownFields::default(),
        }
    }
}