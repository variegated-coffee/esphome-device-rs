use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use crate::api::{EntityCategory, ListEntitiesTextSensorResponse, TextSensorStateResponse};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TextSensorState<'a> {
    pub key: u32,
    pub state: &'a str,
    pub missing_state: bool,
}

impl<'a> Into<TextSensorStateResponse<'a>> for TextSensorState<'a> {
    fn into(self) -> TextSensorStateResponse<'a> {
        TextSensorStateResponse {
            key: self.key,
            state: self.state,
            missing_state: self.missing_state,
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TextSensorConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
    pub device_class: &'a str,
}

impl<'a> Into<ListEntitiesTextSensorResponse<'a>> for &TextSensorConfig<'a> {
    fn into(self) -> ListEntitiesTextSensorResponse<'a> {
        ListEntitiesTextSensorResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            icon: self.icon,
            disabled_by_default: self.disabled_by_default,
            entity_category: Known(self.entity_category),
            device_class: self.device_class,
            unknown_fields: UnknownFields::default(),
        }
    }
}