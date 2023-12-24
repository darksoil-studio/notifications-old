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
