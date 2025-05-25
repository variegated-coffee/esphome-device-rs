use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use crate::api::{ButtonCommandRequest, EntityCategory, ListEntitiesButtonResponse};

#[derive(Clone, PartialEq)]
pub struct ButtonCommandData {
    pub key: u32,
}

impl<'a> From<ButtonCommandRequest<'a>> for ButtonCommandData {
    fn from(request: ButtonCommandRequest<'a>) -> Self {
        ButtonCommandData {
            key: request.key,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ButtonConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
    pub device_class: &'a str,
}

impl<'a> Into<ListEntitiesButtonResponse<'a>> for &ButtonConfig<'a> {
    fn into(self) -> ListEntitiesButtonResponse<'a> {
        ListEntitiesButtonResponse {
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