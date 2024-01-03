use hc_zome_traits::*;
use hdk::prelude::{holo_hash::DnaHash, *};

pub type Hrl = (DnaHash, AnyDhtHash);

#[derive(Serialize, Deserialize, Debug)]
pub struct HrlWithContext {
    pub hrl: Hrl,
    pub context: SerializedBytes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub hrl_to_navigate_to_on_click: HrlWithContext,
    pub pending: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNotificationInput {
    pub notification_hash: AnyDhtHash,
    pub locale: String,
}

#[zome_trait]
pub trait PendingNotifications {
    /// Returning None here means that the notification was not found
    fn get_notification(input: GetNotificationInput) -> ExternResult<Option<Notification>>;

    fn emit_new_pending_notification(notification: Notification) -> ExternResult<()> {
        emit_signal(notification)?;

        Ok(())
    }
}
