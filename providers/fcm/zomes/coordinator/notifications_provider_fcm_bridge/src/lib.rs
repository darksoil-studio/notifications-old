use hc_zome_notifications_provider_fcm_types::RegisterFCMTokenInput;
use hdk::prelude::*;

use hc_zome_notifications_types::NotifyAgentInput;

// TRAIT

#[hdk_extern]
pub fn notify_agent(input: NotifyAgentInput) -> ExternResult<()> {
    let response = call(
        CallTargetCell::OtherRole(String::from("notification_provider")),
        ZomeName::from("fcm_notification_provider"),
        FunctionName::from("notify_agent"),
        None,
        input,
    )?;

    match response {
        ZomeCallResponse::Ok(_) => Ok(()),
        _ => Err(wasm_error!(WasmErrorInner::Guest(format!(
            "Error notifying agent: {response:?}"
        )))),
    }
}

#[hdk_extern]
pub fn register_fcm_token(token: String) -> ExternResult<()> {
    let call_info = call_info()?;
    let response = call(
        CallTargetCell::OtherRole(String::from("notification_provider_fcm")),
        ZomeName::from("notifications_provider_fcm"),
        FunctionName::from("register_fcm_token_for_agent"),
        None,
        RegisterFCMTokenInput {
            token,
            agent: call_info.provenance,
        },
    )?;

    match response {
        ZomeCallResponse::Ok(_) => Ok(()),
        _ => Err(wasm_error!(WasmErrorInner::Guest(format!(
            "Error registering fcm token: {response:?}"
        )))),
    }
}
