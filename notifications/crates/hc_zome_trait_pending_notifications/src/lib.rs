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
    notification_hash: AnyDhtHash,
    locale: String,
}

#[zome_trait]
pub trait PendingNotifications {
    fn get_notification(input: GetNotificationInput) -> ExternResult<Option<Notification>>;

    fn emit_new_pending_notification(
        pending_notification: PendingNotification,
    ) -> ExternResult<()> {
        emit_signal(pending_notification)?;

        Ok(())
    }
}
