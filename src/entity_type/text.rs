use alloc::string::String;
use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use crate::api::{EntityCategory, ListEntitiesTextResponse, TextCommandRequest, TextMode, TextStateResponse};

#[derive(Clone, PartialEq)]
pub struct TextCommandData {
    pub key: u32,
    pub state: String,
}

impl<'a> From<TextCommandRequest<'a>> for TextCommandData {
    fn from(request: TextCommandRequest<'a>) -> Self {
        TextCommandData {
            key: request.key,
            state: String::try_from(request.state).unwrap_or_default(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TextConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
    pub min_length: u32,
    pub max_length: u32,
    pub pattern: &'a str,
    pub mode: TextMode,
}

impl<'a> Into<ListEntitiesTextResponse<'a>> for &TextConfig<'a> {
    fn into(self) -> ListEntitiesTextResponse<'a> {
        ListEntitiesTextResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            icon: self.icon,
            disabled_by_default: self.disabled_by_default,
            entity_category: Known(self.entity_category),
            min_length: self.min_length,
            max_length: self.max_length,
            pattern: self.pattern,
            mode: Known(self.mode),
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct TextState<'a> {
    pub key: u32,
    pub state: &'a str,
    pub missing_state: bool,
}

impl<'a> Into<TextStateResponse<'a>> for TextState<'a> {
    fn into(self) -> TextStateResponse<'a> {
        TextStateResponse {
            key: self.key,
            state: self.state,
            missing_state: self.missing_state,
            unknown_fields: UnknownFields::default(),
        }
    }
}