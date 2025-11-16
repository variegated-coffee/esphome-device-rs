use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use crate::api::{AlarmControlPanelCommandRequest, AlarmControlPanelState, AlarmControlPanelStateCommand, AlarmControlPanelStateResponse, EntityCategory, ListEntitiesAlarmControlPanelResponse};

#[derive(Clone, PartialEq)]
pub struct AlarmControlPanelCommandData {
    pub key: u32,
    pub command: Option<AlarmControlPanelStateCommand>,
    pub code: String,
}

impl<'a> From<AlarmControlPanelCommandRequest<'a>> for AlarmControlPanelCommandData {
    fn from(request: AlarmControlPanelCommandRequest<'a>) -> Self {
        AlarmControlPanelCommandData {
            key: request.key,
            command: match request.command {
                Known(cmd) => Some(cmd),
                _ => None,
            },
            code: String::try_from(request.code).unwrap_or_default(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AlarmControlPanelConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
    pub supported_features: u32,
    pub requires_code: bool,
    pub requires_code_to_arm: bool,
}

impl<'a> Into<ListEntitiesAlarmControlPanelResponse<'a>> for &AlarmControlPanelConfig<'a> {
    fn into(self) -> ListEntitiesAlarmControlPanelResponse<'a> {
        ListEntitiesAlarmControlPanelResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            icon: self.icon,
            disabled_by_default: self.disabled_by_default,
            entity_category: Known(self.entity_category),
            supported_features: self.supported_features,
            requires_code: self.requires_code,
            requires_code_to_arm: self.requires_code_to_arm,
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct AlarmControlPanelEntityState {
    pub key: u32,
    pub state: AlarmControlPanelState,
}

impl<'a> Into<AlarmControlPanelStateResponse<'a>> for AlarmControlPanelEntityState {
    fn into(self) -> AlarmControlPanelStateResponse<'a> {
        AlarmControlPanelStateResponse {
            key: self.key,
            state: Known(self.state),
            unknown_fields: UnknownFields::default(),
        }
    }
}