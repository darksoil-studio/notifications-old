use hdk::prelude::*;

#[hdk_extern]
pub fn register_new_fcm_token(token: String) -> ExternResult<()> {
    let response = call(
        CallTargetCell::Local,
        "notifications",
        "get_available_notification_provider".into(),
        None,
        (),
    )?;

    let ZomeCallResponse::Ok(result ) = response else {
        return Err(wasm_error!(WasmErrorInner::Guest(format!("Failed to get available provider {response:?}"))));
    };

    let maybe_provider: Option<AgentPubKey> = result.decode().map_err(|err| wasm_error!(err))?;

    let Some(provider) = maybe_provider else {
        return Err(wasm_error!(WasmErrorInner::Guest(format!("There is no provider available"))));
    };

    let response = call_remote(
        provider,
        "notifications_provider_fcm_bridge",
        "register_fcm_token".into(),
        None,
        token,
    )?;

    match response {
        ZomeCallResponse::Ok(_) => Ok(()),
        _ => Err(wasm_error!(WasmErrorInner::Guest(format!(
            "Error registering fcm token: {response:?}"
        )))),
    }
}
