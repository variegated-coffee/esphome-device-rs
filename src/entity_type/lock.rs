use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use crate::api::{EntityCategory, ListEntitiesLockResponse, LockCommand, LockCommandRequest, LockState, LockStateResponse};

#[derive(Clone, PartialEq)]
pub struct LockCommandData {
    pub key: u32,
    pub command: Option<LockCommand>,
    pub code: Option<String>,
}

impl<'a> From<LockCommandRequest<'a>> for LockCommandData {
    fn from(request: LockCommandRequest<'a>) -> Self {
        LockCommandData {
            key: request.key,
            command: match request.command {
                Known(cmd) => Some(cmd),
                _ => None,
            },
            code: if request.has_code {
                String::try_from(request.code).ok()
            } else {
                None
            },
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct LockConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
    pub assumed_state: bool,
    pub supports_open: bool,
    pub requires_code: bool,
    pub code_format: &'a str,
}

impl<'a> Into<ListEntitiesLockResponse<'a>> for &LockConfig<'a> {
    fn into(self) -> ListEntitiesLockResponse<'a> {
        ListEntitiesLockResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            icon: self.icon,
            disabled_by_default: self.disabled_by_default,
            entity_category: Known(self.entity_category),
            assumed_state: self.assumed_state,
            supports_open: self.supports_open,
            requires_code: self.requires_code,
            code_format: self.code_format,
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct LockEntityState {
    pub key: u32,
    pub state: LockState,
}

impl<'a> Into<LockStateResponse<'a>> for LockEntityState {
    fn into(self) -> LockStateResponse<'a> {
        LockStateResponse {
            key: self.key,
            state: Known(self.state),
            unknown_fields: UnknownFields::default(),
        }
    }
}