use hc_zome_notifications_provider_types::{NotifyAgentInput, NotifyAgentSignal};
use hdk::prelude::*;

mod fcm_tokens;
mod service_account_keys;

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
