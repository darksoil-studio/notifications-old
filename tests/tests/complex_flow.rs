use std::collections::BTreeMap;

use hdk::prelude::*;
use holochain::test_utils::consistency_10s;
use holochain::{conductor::config::ConductorConfig, sweettest::*};

#[tokio::test(flavor = "multi_thread")]
async fn complex_flow() {
    // Use prebuilt DNA file
    let notifications_dna = SweetDnaFile::from_bundle(
        &std::env::current_dir()
            .unwrap()
            .join("../notifications/workdir/notifications.dna"),
    )
    .await
    .unwrap();
    let fixture_app = [notifications_dna];

    let fcm_notifications_dna= SweetDnaFile::from_bundle(&std::env::current_dir()
        .unwrap()
        .join("../providers/fcm/apps/notifications_provider_fcm/dnas/notifications/workdir/notifications.dna")).await.unwrap();
    let fcm_provider_dna= SweetDnaFile::from_bundle(&std::env::current_dir()
        .unwrap()
        .join("../providers/fcm/apps/notifications_provider_fcm/dnas/notifications_provider_fcm/workdir/notifications_provider_fcm.dna")).await.unwrap();
    let fcm_provider_app = [fcm_notifications_dna, fcm_provider_dna];

    let fcm_recipient_dna= SweetDnaFile::from_bundle(&std::env::current_dir()
        .unwrap()
        .join("../providers/fcm/apps/notifications_provider_fcm_recipient/workdir/notifications_fcm_recipient.dna")).await.unwrap();
    let fcm_recipient_app = [fcm_recipient_dna];

    // Set up conductors
    let mut conductors = SweetConductorBatch::from_config(3, ConductorConfig::default()).await;

    let fixture_app = conductors[0]
        .setup_app("gather", &fixture_app)
        .await
        .unwrap();
    let provider_app = conductors[1]
        .setup_app("fcm_provider", &fcm_provider_app)
        .await
        .unwrap();
    let recipient_app = conductors[2]
        .setup_app("fcm_recipient", &fcm_recipient_app)
        .await
        .unwrap();
    conductors.exchange_peer_info().await;

    let conductors = conductors.into_inner();

    let fixture = &conductors[0];
    let provider = &conductors[1];
    let recipient = &conductors[2];

    let fixture_alice = fixture_app.into_cells();
    let fixture_zome = fixture_alice[0].zome("notifications");

    let provider_cells = provider_app.into_cells();
    let provider_notifications_zome = provider_cells[0].zome("notifications");
    let provider_fcm_zome = provider_cells[1].zome("notifications_provider_fcm");

    let recipient_cells = recipient_app.into_cells();
    let recipient_zome = recipient_cells[0].zome("notifications");

    /* Setup provider */

    // Publish Service Account Key
    // Announce as provider

    /* Setup recipient */
    // Register FCM token
    // Shutdown recipient

    /* Send notification */
    // Send notification from fixture notification zome
    // FCM provider zome sends signal
    // Turn on recipient again

    // let record_1: Option<Record> = conductors[0]
    //     .call(&alice_zome, "get_agent_profile", alice_pub_key)
    //     .await;
    // assert_eq!(record_1, None);

    // consistency_10s([&alice, &bobbo]).await;
}
