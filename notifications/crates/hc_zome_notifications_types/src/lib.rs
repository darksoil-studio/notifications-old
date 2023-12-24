use hdi::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct NotifyAgentInput {
    pub notification: SerializedBytes,
    pub agent: AgentPubKey,
}
