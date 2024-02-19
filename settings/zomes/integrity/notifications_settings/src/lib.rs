use std::collections::HashMap;

use hdi::prelude::*;

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum NotificationProviderSettings {
    Email { email_address: String },
}

#[hdk_entry_helper]
pub struct NotificationsSettings {
    settings_by_notification_type: HashMap<String, NotificationProviderSettings>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    NotificationsSettings(NotificationsSettings),
}

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    AgentToNotificationsSettings,
}
