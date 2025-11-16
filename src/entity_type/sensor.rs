use femtopb::EnumValue::Known;
use femtopb::UnknownFields;
use crate::api::{EntityCategory, ListEntitiesSensorResponse, SensorLastResetType, SensorStateClass, SensorStateResponse};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SensorState {
    pub key: u32,
    pub state: f32,
    pub missing_state: bool,
}

impl SensorState {
    pub fn new(key: u32, state: Option<f32>) -> Self {
        if let Some(state) = state {
            Self {
                key,
                state,
                missing_state: false,
            }
        } else {
            Self {
                key,
                state: 0.0,
                missing_state: true,
            }
        }
    }
}

impl<'a> Into<SensorStateResponse<'a>> for SensorState {
    fn into(self) -> SensorStateResponse<'a> {
        SensorStateResponse {
            key: self.key,
            state: self.state,
            missing_state: self.missing_state,
            unknown_fields: UnknownFields::default(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SensorConfig<'a> {
    pub object_id: &'a str,
    pub key: u32,
    pub name: &'a str,
    pub unique_id: &'a str,
    pub icon: &'a str,
    pub unit_of_measurement: &'a str,
    pub accuracy_decimals: i32,
    pub force_update: bool,
    pub device_class: &'a str,
    pub state_class: SensorStateClass,
    pub disabled_by_default: bool,
    pub entity_category: EntityCategory,
}

impl<'a> Into<ListEntitiesSensorResponse<'a>> for &SensorConfig<'a> {
    fn into(self) -> ListEntitiesSensorResponse<'a> {
        ListEntitiesSensorResponse {
            object_id: self.object_id,
            key: self.key,
            name: self.name,
            unique_id: self.unique_id,
            icon: self.icon,
            unit_of_measurement: self.unit_of_measurement,
            accuracy_decimals: self.accuracy_decimals,
            force_update: self.force_update,
            device_class: self.device_class,
            state_class: Known(self.state_class),
            disabled_by_default: self.disabled_by_default,
            entity_category: Known(self.entity_category),
            legacy_last_reset_type: Known(SensorLastResetType::LastResetNever),
            unknown_fields: UnknownFields::default(),
        }
    }
}