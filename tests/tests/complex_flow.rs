use std::collections::BTreeMap;

use hdk::prelude::*;
use holochain::test_utils::consistency_10s;
use holochain::{conductor::config::ConductorConfig, sweettest::*};

#[tokio::test(flavor = "multi_thread")]
async fn create_and_get() {
    // Use prebuilt DNA file
    let dna_path = std::env::current_dir()
        .unwrap()
        .join("../../workdir/profiles-test.dna");
    let dna = SweetDnaFile::from_bundle(&dna_path).await.unwrap();

    // Set up conductors
    let mut conductors = SweetConductorBatch::from_config(2, ConductorConfig::default()).await;
    let apps = conductors.setup_app("profiles", &[dna]).await.unwrap();
    conductors.exchange_peer_info().await;

    let ((alice,), (bobbo,)) = apps.into_tuples();

    let alice_zome = alice.zome("profiles");
    let bob_zome = bobbo.zome("profiles");

    let alice_pub_key = alice.agent_pubkey();

    // Try to get my profile before creating one. Should return None.
    let record_1: Option<Record> = conductors[0]
        .call(&alice_zome, "get_agent_profile", alice_pub_key)
        .await;
    assert_eq!(record_1, None);

    // Create profile for alice and try to get it via get_my_profile() as well as get_agent_profile()
    let profile = Profile {
        nickname: String::from("alice"),
        fields: BTreeMap::new(),
    };

    let record_1: Record = conductors[0]
        .call(&alice_zome, "create_profile", profile)
        .await;

    consistency_10s([&alice, &bobbo]).await;

}
