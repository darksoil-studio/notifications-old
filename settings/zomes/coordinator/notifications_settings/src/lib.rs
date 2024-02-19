use hc_zome_notifications_settings_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn set_notifications_settings(settings: NotificationsSettings) -> ExternResult<()> {
    let my_pub_key = agent_info()?.agent_latest_pubkey;
    let links = get_notifications_settings_links_for_agent(my_pub_key.clone())?;

    for link in links {
        delete_link(link.create_link_hash)?;
    }

    let action_hash = create_entry(EntryTypes::NotificationsSettings(settings))?;

    create_link(
        my_pub_key,
        action_hash,
        LinkTypes::AgentToNotificationsSettings,
        (),
    )?;

    Ok(())
}

pub fn get_notifications_settings_links_for_agent(agent: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(agent, LinkTypes::AgentToNotificationsSettings)?.build(),
    )
}

#[hdk_extern]
pub fn get_notifications_settings_for(agent: AgentPubKey) -> ExternResult<Option<Record>> {
    let links = get_notifications_settings_links_for_agent(agent)?;

    let Some(link) = links.first() else {
        return Ok(None); // Agent hasn't created any notifiations settings yet
    };

    let action_hash =
        link.target
            .clone()
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "Malformed notifications settings link".into()
            )))?;

    get(action_hash, Default::default())
}
