use hc_zome_notifications_integrity::*;
use hc_zome_notifications_types::NotifyAgentInput;
use hdk::prelude::*;

fn providers_path() -> Path {
    Path::from("notifications_providers")
}

#[hdk_extern]
pub fn announce_as_provider(_: ()) -> ExternResult<()> {
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    let mut functions: BTreeSet<(ZomeName, FunctionName)> = BTreeSet::new();

    let zome_name = zome_info()?.name;
    functions.insert((zome_name.clone(), FunctionName::from("ping")));
    functions.insert((zome_name.clone(), FunctionName::from("notify_agent")));

    create_cap_grant(ZomeCallCapGrant {
        tag: String::from("notifications_provider"),
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(functions),
    })?;

    create_link(
        providers_path().path_entry_hash()?,
        my_pub_key,
        LinkTypes::NotificationsProvider,
        (),
    )?;

    Ok(())
}

#[hdk_extern]
pub fn ping(_: ()) -> ExternResult<()> {
    Ok(())
}

fn get_all_notifications_provider() -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(
        providers_path().path_entry_hash()?,
        LinkTypes::NotificationsProvider,
        None,
    )?;

    let pubkeys = links
        .into_iter()
        .filter_map(|l| l.target.into_agent_pub_key())
        .collect();

    Ok(pubkeys)
}

pub fn get_available_notification_provider() -> ExternResult<Option<AgentPubKey>> {
    let all_providers = get_all_notifications_provider()?;

    for provider in all_providers {
        let result = call_remote(
            provider.clone(),
            zome_info()?.name,
            FunctionName::from("ping"),
            None,
            (),
        )?;

        if let ZomeCallResponse::Ok(_) = result {
            return Ok(Some(provider));
        }
    }

    Ok(None)
}

#[hdk_extern]
pub fn request_notify_agent(input: NotifyAgentInput) -> ExternResult<()> {
    let Some(provider) = get_available_notification_provider()? else {
        return Err(wasm_error!(WasmErrorInner::Guest(String::from("Can't find any notifications provider"))))?;
    };

    let response = call_remote(
        provider.clone(),
        zome_info()?.name,
        FunctionName::from("notify_agent"), // Needs to be defined in the provider's coordinator zome
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
