use crate::{create_link_relaxed, delete_link_relaxed};
use hc_zome_notifications_provider_fcm_integrity::LinkTypes;
use hc_zome_notifications_provider_fcm_types::RegisterFCMTokenInput;
use hdk::prelude::*;

#[hdk_extern]
pub fn register_fcm_token_for_agent(input: RegisterFCMTokenInput) -> ExternResult<()> {
    let links = get_links(
        GetLinksInputBuilder::try_new(input.agent.clone(), LinkTypes::FCMToken)?.build(),
    )?;

    for link in links {
        delete_link_relaxed(link.create_link_hash)?;
    }

    create_link_relaxed(
        input.agent.clone(),
        input.agent,
        LinkTypes::FCMToken,
        input.token.as_bytes().to_vec(),
    )?;

    Ok(())
}

pub fn get_fcm_token_for_agent(agent: AgentPubKey) -> ExternResult<Option<String>> {
    let links =
        get_links(GetLinksInputBuilder::try_new(agent.clone(), LinkTypes::FCMToken)?.build())?;

    let Some(link) = links.first().cloned() else {
        return Ok(None);
    };

    let token = String::from_utf8(link.tag.into_inner()).map_err(|err| {
        wasm_error!(WasmErrorInner::Guest(format!(
            "Malformed token tag {err:?}"
        )))
    })?;

    Ok(Some(token))
}
