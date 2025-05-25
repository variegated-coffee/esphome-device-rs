use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use crate::api::{BinarySensorStateResponse, EntityCategory, ListEntitiesBinarySensorResponse};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BinarySensorState {
    pub key: u32,
    pub state: bool,
    pub missing_state: bool,
}

impl<'a> Into<BinarySensorStateResponse<'a>> for BinarySensorState {
    fn into(self) -> BinarySensorStateResponse<'a> {
        BinarySensorStateResponse {
            key: self.key,
            state: self.state,
            missing_state: self.missing_state,
            unknown_fields: UnknownFields::default(),
        }
    }
}

pub struct BinarySensorConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub device_class: &'a str,
    pub is_status_binary_sensor: bool,
    pub disabled_by_default: bool,
    pub icon: &'a str,
    pub entity_category: EntityCategory,
}

impl<'a> Into<ListEntitiesBinarySensorResponse<'a>> for &BinarySensorConfig<'a> {
    fn into(self) -> ListEntitiesBinarySensorResponse<'a> {
        ListEntitiesBinarySensorResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            device_class: self.device_class,
            is_status_binary_sensor: self.is_status_binary_sensor,
            disabled_by_default: self.disabled_by_default,
            icon: self.icon,
            entity_category: Known(self.entity_category),
            unknown_fields: UnknownFields::default(),
        }
    }
}
