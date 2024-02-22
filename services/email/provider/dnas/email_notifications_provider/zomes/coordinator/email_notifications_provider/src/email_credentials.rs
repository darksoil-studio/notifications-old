use crate::{create_link_relaxed, delete_link_relaxed};
use email_notifications_provider_integrity::*;
use email_notifications_types::EmailCredentials;
use hdk::prelude::*;

fn email_credentials_path() -> Path {
    Path::from("email_credentials")
}

#[hdk_extern]
pub fn publish_new_email_credentials(credentials: EmailCredentials) -> ExternResult<()> {
    let links = get_links(
        GetLinksInputBuilder::try_new(
            email_credentials_path().path_entry_hash()?,
            LinkTypes::EmailCredentials,
        )?
        .build(),
    )?;

    for link in links {
        delete_link_relaxed(link.create_link_hash)?;
    }

    let action_hash = create_entry(EntryTypes::EmailCredentials(credentials))?;

    create_link_relaxed(
        email_credentials_path().path_entry_hash()?,
        action_hash,
        LinkTypes::EmailCredentials,
        (),
    )?;

    Ok(())
}

pub fn get_current_email_credentials() -> ExternResult<Option<EmailCredentials>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(
            email_credentials_path().path_entry_hash()?,
            LinkTypes::EmailCredentials,
        )?
        .build(),
    )?;

    let Some(link) = links.first().cloned() else {
        return Ok(None);
    };

    let Some(record) = get(
        link.target
            .into_any_dht_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Malformed link"
            ))))?,
        GetOptions::default(),
    )?
    else {
        return Ok(None);
    };

    let credentials: EmailCredentials = record
        .entry()
        .as_option()
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Malformed key"
        ))))?
        .try_into()?;

    Ok(Some(credentials))
}
