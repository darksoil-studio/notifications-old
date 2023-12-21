use hc_zome_fcm_push_notifications_provider_types::ServiceAccountKey;
use hdi::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    ServiceAccountKey(ServiceAccountKey),
}

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    ServiceAccountKeys,
    FCMToken,
}
