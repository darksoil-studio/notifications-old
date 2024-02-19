use hc_zome_email_notifications_types::{SendEmailInput, SendEmailSignal};
use hdk::prelude::*;

mod email_credentials;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let response = call(
        CallTargetCell::OtherRole("email_notifications_service".into()),
        ZomeName::from("email_notifications_service"),
        "announce_as_provider".into(),
        None,
        (),
    )?;
    let ZomeCallResponse::Ok(_result) = response else {
        return Err(wasm_error!(WasmErrorInner::Guest(format!(
            "Failed to announce as provider {response:?}"
        ))));
    };

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn send_email(input: SendEmailInput) -> ExternResult<()> {
    let Some(credentials) = email_credentials::get_current_email_credentials()? else {
        return Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "email authority hasn't registered a service account key yet"
        ))));
    };

    let signal = SendEmailSignal {
        email: input.email,
        email_address: input.email_address,
        credentials,
    };

    emit_signal(signal)?;

    Ok(())
}

/** Utils **/

pub fn delete_link_relaxed(address: ActionHash) -> ExternResult<ActionHash> {
    HDK.with(|h| {
        h.borrow()
            .delete_link(DeleteLinkInput::new(address, ChainTopOrdering::Relaxed))
    })
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
