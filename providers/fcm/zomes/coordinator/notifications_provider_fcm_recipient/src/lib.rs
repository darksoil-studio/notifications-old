use hdk::prelude::*;

#[hdk_extern]
pub fn register_new_fcm_token(token: String) -> ExternResult<()> {
    Ok(())
}
