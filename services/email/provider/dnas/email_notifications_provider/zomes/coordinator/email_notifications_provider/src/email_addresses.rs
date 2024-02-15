use crate::{create_link_relaxed, delete_link_relaxed};
use hc_zome_email_notifications_provider_integrity::LinkTypes;
use hc_zome_email_notifications_types::RegisterEmailAddressInput;
use hdk::prelude::*;

#[hdk_extern]
pub fn register_email_address_for_agent(input: RegisterEmailAddressInput) -> ExternResult<()> {
    let links = get_links(
        GetLinksInputBuilder::try_new(input.agent.clone(), LinkTypes::EmailAddresses)?.build(),
    )?;

    for link in links {
        delete_link_relaxed(link.create_link_hash)?;
    }

    create_link_relaxed(
        input.agent.clone(),
        input.agent,
        LinkTypes::EmailAddresses,
        input.email_address.as_bytes().to_vec(),
    )?;

    Ok(())
}

pub fn get_email_address_for_agent(agent: AgentPubKey) -> ExternResult<Option<String>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(agent.clone(), LinkTypes::EmailAddresses)?.build(),
    )?;

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
