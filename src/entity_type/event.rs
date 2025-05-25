use femtopb::EnumValue::Known;
use femtopb::{Repeated, UnknownFields};
use crate::api::{EntityCategory, EventResponse, ListEntitiesEventResponse};

#[derive(Clone, PartialEq)]
pub struct EventConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
    pub device_class: &'a str,
    pub event_types: &'a [&'a str],
}

impl<'a> Into<ListEntitiesEventResponse<'a>> for &EventConfig<'a> {
    fn into(self) -> ListEntitiesEventResponse<'a> {
        ListEntitiesEventResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            icon: self.icon,
            disabled_by_default: self.disabled_by_default,
            entity_category: Known(self.entity_category),
            device_class: self.device_class,
            event_types: Repeated::from(self.event_types),
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct EventState<'a> {
    pub key: u32,
    pub event_type: &'a str,
}

impl<'a> Into<EventResponse<'a>> for EventState<'a> {
    fn into(self) -> EventResponse<'a> {
        EventResponse {
            key: self.key,
            event_type: self.event_type,
            unknown_fields: UnknownFields::default(),
        }
    }
}