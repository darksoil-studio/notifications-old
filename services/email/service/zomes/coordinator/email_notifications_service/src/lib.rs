use hc_zome_email_notifications_service_integrity::*;
use hc_zome_email_notifications_types::SendEmailInput;
use hdk::prelude::*;

fn providers_path() -> Path {
    Path::from("notifications_providers")
}

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut functions: BTreeSet<(ZomeName, FunctionName)> = BTreeSet::new();

    let zome_name = zome_info()?.name;
    functions.insert((zome_name.clone(), FunctionName::from("ping")));

    create_cap_grant(ZomeCallCapGrant {
        tag: String::from("ping"),
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(functions),
    })?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn announce_as_provider(_: ()) -> ExternResult<()> {
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    let mut functions: BTreeSet<(ZomeName, FunctionName)> = BTreeSet::new();

    let zome_name = zome_info()?.name;
    functions.insert((zome_name.clone(), FunctionName::from("ping")));
    functions.insert((zome_name.clone(), FunctionName::from("send_email")));

    create_cap_grant(ZomeCallCapGrant {
        tag: String::from("notifications_provider"),
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(functions),
    })?;

    create_link_relaxed(
        providers_path().path_entry_hash()?,
        my_pub_key,
        LinkTypes::NotificationsProvider,
        (),
    )?;

    Ok(())
}

pub fn create_link_relaxed<T, E>(
    base_address: impl Into<AnyLinkableHash>,
    target_address: impl Into<AnyLinkableHash>,
    link_type: T,
    tag: impl Into<LinkTag>,
) -> ExternResult<ActionHash>
where
    ScopedLinkType: TryFrom<T, Error = E>,
    WasmError: From<E>,
{
    let ScopedLinkType {
        zome_index,
        zome_type: link_type,
    } = link_type.try_into()?;
    HDK.with(|h| {
        h.borrow().create_link(CreateLinkInput::new(
            base_address.into(),
            target_address.into(),
            zome_index,
            link_type,
            tag.into(),
            ChainTopOrdering::Relaxed,
        ))
    })
}

#[hdk_extern]
pub fn ping(_: ()) -> ExternResult<()> {
    Ok(())
}

fn get_all_notifications_providers() -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(
            providers_path().path_entry_hash()?,
            LinkTypes::NotificationsProvider,
        )?
        .build(),
    )?;

    let pubkeys = links
        .into_iter()
        .filter_map(|l| l.target.into_agent_pub_key())
        .collect();

    Ok(pubkeys)
}

#[hdk_extern]
pub fn get_available_notification_providers(_: ()) -> ExternResult<Option<AgentPubKey>> {
    let all_providers = get_all_notifications_providers()?;

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
pub fn register_email_address(email_address: String) -> ExternResult<()> {
    let Some(provider) = get_available_notification_providers(())? else {
        return Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Can't find any notifications provider"
        ))))?;
    };

    let response = call_remote(
        provider.clone(),
        "email_notifications_bridge",
        FunctionName::from("register_email_address"),
        None,
        email_address,
    )?;

    match response {
        ZomeCallResponse::Ok(_) => Ok(()),
        _ => Err(wasm_error!(WasmErrorInner::Guest(format!(
            "Error registering email address: {response:?}"
        )))),
    }
}

#[hdk_extern]
pub fn request_send_email(input: SendEmailInput) -> ExternResult<()> {
    let Some(provider) = get_available_notification_providers(())? else {
        return Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Can't find any notifications provider"
        ))))?;
    };

    let response = call_remote(
        provider.clone(),
        "email_notifications_bridge",
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
