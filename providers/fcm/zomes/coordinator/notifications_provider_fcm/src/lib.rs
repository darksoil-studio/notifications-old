use hc_zome_notifications_provider_fcm_types::NotifyAgentSignal;
use hc_zome_notifications_types::NotifyAgentInput;
use hdk::prelude::*;

mod fcm_tokens;
mod service_account_keys;

pub fn delete_link_relaxed(address: ActionHash) -> ExternResult<ActionHash> {
    HDK.with(|h| {
        h.borrow()
            .delete_link(DeleteLinkInput::new(address, ChainTopOrdering::Relaxed))
    })
}

pub fn create_link_relaxed<T, E>(
    base_address: impl Into<AnyLinkableHash>,
    target_address: impl Into<AnyLinkableHash>,
    link_type: T,
    tag: impl Into<LinkTag>,
) -> ExternResult<ActionHash>
where
    ScopedLinkType: TryFrom<T, Error = E>,
    WasmError: From<E>,
{
    let ScopedLinkType {
        zome_index,
        zome_type: link_type,
    } = link_type.try_into()?;
    HDK.with(|h| {
        h.borrow().create_link(CreateLinkInput::new(
            base_address.into(),
            target_address.into(),
            zome_index,
            link_type,
            tag.into(),
            ChainTopOrdering::Relaxed,
        ))
    })
}

#[hdk_extern]
pub fn notify_agent(input: NotifyAgentInput) -> ExternResult<()> {
    let Some(token ) = fcm_tokens::get_fcm_token_for_agent(input.agent)? else {
        return Err(wasm_error!(WasmErrorInner::Guest(String::from("Agent hasn't registered their FCM token yet"))));
    };

    let Some(service_account_key)= service_account_keys::get_current_service_account_key()?  else {
                return Err(wasm_error!(WasmErrorInner::Guest(String::from("FCM authority hasn't registered a service account key yet"))));
    };

    let signal = NotifyAgentSignal {
        notification: input.notification,
        token,
        service_account_key,
    };

    emit_signal(signal)?;

    Ok(())
}
