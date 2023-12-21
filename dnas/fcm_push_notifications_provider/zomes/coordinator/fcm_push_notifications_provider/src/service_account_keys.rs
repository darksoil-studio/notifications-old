use fcm_notifications_provider_integrity::*;
use hc_zome_notifications_provider_types::ServiceAccountKey;
use hdk::prelude::*;

fn service_account_key_path() -> Path {
    Path::from("service_account_keys")
}

#[hdk_extern]
pub fn publish_new_service_account_key(key: ServiceAccountKey) -> ExternResult<()> {
    let links = get_links(
        service_account_key_path().path_entry_hash()?,
        LinkTypes::ServiceAccountKeys,
        None,
    )?;

    for link in links {
        delete_link(link.create_link_hash)?;
    }

    let action_hash = create_entry(EntryTypes::ServiceAccountKey(key))?;

    create_link(
        service_account_key_path().path_entry_hash()?,
        action_hash,
        LinkTypes::ServiceAccountKeys,
        (),
    )?;

    Ok(())
}

pub fn get_current_service_account_key() -> ExternResult<Option<ServiceAccountKey>> {
    let links = get_links(
        service_account_key_path().path_entry_hash()?,
        LinkTypes::ServiceAccountKeys,
        None,
    )?;

    let Some(link) = links.first().cloned() else {
        return Ok(None);
    };

    let Some(record)  = get(link.target.into_any_dht_hash().ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Malformed link"))))?, GetOptions ::default())? else {
        return Ok(None);
    };

    let key: ServiceAccountKey = record
        .entry()
        .as_option()
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Malformed key"
        ))))?
        .try_into()?;

    Ok(Some(key))
}
