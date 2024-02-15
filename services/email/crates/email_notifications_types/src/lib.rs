use hdi::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendEmailInput {
    pub agent: AgentPubKey,
    pub email: Email,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
    pub subject: String,
    pub body: String,
}

#[derive(Clone)]
#[hdk_entry_helper]
pub struct EmailCredentials {
    pub username: String,
    pub password: String,
    pub smtp_relay_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendEmailSignal {
    pub email_address: String,
    pub email: Email,
    pub credentials: EmailCredentials,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterEmailAddressInput {
    pub email_address: String,
    pub agent: AgentPubKey,
}
