use hc_zome_email_notifications_types::EmailCredentials;
use hdi::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    EmailCredentials(EmailCredentials),
}

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    EmailCredentials,
    EmailAddresses,
}
