use anyhow::anyhow;
use femtopb::Message;
use crate::metadata::MessageType;

#[derive(Default)]
pub struct ConnectionStatus {
    pub authenticated: bool,
    pub setup_complete: bool,
    pub subscribed_to_states: bool,
    pub subscribed_to_logs: bool,
}