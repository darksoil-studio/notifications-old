use hdi::prelude::*;

/// JSON schema of secret service account key.
///
/// You can obtain the key from the [Cloud Console](https://console.cloud.google.com/).
///
/// You can use `helpers::read_service_account_key()` as a quick way to read a JSON client
/// secret into a ServiceAccountKey.
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ServiceAccountKey {
    #[serde(rename = "type")]
    /// key_type
    pub key_type: Option<String>,
    /// project_id
    pub project_id: Option<String>,
    /// private_key_id
    pub private_key_id: Option<String>,
    /// private_key
    pub private_key: String,
    /// client_email
    pub client_email: String,
    /// client_id
    pub client_id: Option<String>,
    /// auth_uri
    pub auth_uri: Option<String>,
    /// token_uri
    pub token_uri: String,
    /// auth_provider_x509_cert_url
    pub auth_provider_x509_cert_url: Option<String>,
    /// client_x509_cert_url
    pub client_x509_cert_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotifyAgentSignal {
    pub notification: SerializedBytes,
    pub token: String,
    pub service_account_key: ServiceAccountKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterFCMTokenInput {
    pub token: String,
    pub agent: AgentPubKey,
}
