use hc_zome_fcm_notifications_types::{NotifyAgentInput, RegisterFCMTokenInput};
use hdk::prelude::*;

// TRAIT

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut functions: BTreeSet<(ZomeName, FunctionName)> = BTreeSet::new();

    let zome_name = zome_info()?.name;
    functions.insert((zome_name.clone(), FunctionName::from("register_fcm_token")));
    functions.insert((zome_name.clone(), FunctionName::from("notify_agent")));

    create_cap_grant(ZomeCallCapGrant {
        tag: String::from("notifications_provider"),
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(functions),
    })?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn notify_agent(input: NotifyAgentInput) -> ExternResult<()> {
    let response = call(
        CallTargetCell::OtherRole(String::from("notifications_provider_fcm")),
        ZomeName::from("notifications_provider_fcm"),
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
        CallTargetCell::OtherRole(String::from("notifications_provider_fcm")),
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
