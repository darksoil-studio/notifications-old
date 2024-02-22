use email_notifications_types::SendEmailInput;
use hdk::prelude::*;

// TRAIT

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut functions: BTreeSet<(ZomeName, FunctionName)> = BTreeSet::new();

    let zome_name = zome_info()?.name;
    functions.insert((zome_name.clone(), FunctionName::from("send_email")));

    create_cap_grant(ZomeCallCapGrant {
        tag: String::from("notifications_provider"),
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(functions),
    })?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn send_email(input: SendEmailInput) -> ExternResult<()> {
    let response = call(
        CallTargetCell::OtherRole(String::from("email_notifications_provider")),
        ZomeName::from("email_notifications_provider"),
        FunctionName::from("send_email"),
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
