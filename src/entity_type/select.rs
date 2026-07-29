use alloc::string::String;
use femtopb::{Repeated, UnknownFields};
use femtopb::EnumValue::Known;
use crate::api::{EntityCategory, ListEntitiesSelectResponse, SelectCommandRequest, SelectStateResponse};

#[derive(Clone, PartialEq)]
pub struct SelectCommandData {
    pub key: u32,
    pub state: String,
}

impl<'a> From<SelectCommandRequest<'a>> for SelectCommandData {
    fn from(request: SelectCommandRequest<'a>) -> Self {
        SelectCommandData {
            key: request.key,
            state: String::try_from(request.state).unwrap_or_default(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SelectConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub options: &'a[&'a str],
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
}

impl<'a> Into<ListEntitiesSelectResponse<'a>> for &SelectConfig<'a> {
    fn into(self) -> ListEntitiesSelectResponse<'a> {
        ListEntitiesSelectResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            icon: self.icon,
            options: Repeated::from(self.options),
            disabled_by_default: self.disabled_by_default,
            entity_category: Known(self.entity_category),
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct SelectState<'a> {
    pub key: u32,
    pub state: &'a str,
    pub missing_state: bool,
}

impl<'a> Into<SelectStateResponse<'a>> for SelectState<'a> {
    fn into(self) -> SelectStateResponse<'a> {
        SelectStateResponse {
            key: self.key,
            state: self.state,
            missing_state: self.missing_state,
            unknown_fields: UnknownFields::default(),
        }
    }
}