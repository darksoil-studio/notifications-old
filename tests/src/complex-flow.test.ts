import { test, assert } from 'vitest';
import { dirname } from 'path';
import { fileURLToPath } from 'url';

import { runScenario, dhtSync, pause } from '@holochain/tryorama';
import { AppAgentWebsocket } from '@holochain/client';
import { encode } from '@msgpack/msgpack';

test('setup provider and recipient, and send a notification to it', async t => {
  await runScenario(
    async scenario => {
      const testHappUrl =
        dirname(fileURLToPath(import.meta.url)) +
        '/../fixture/workdir/test_app.happ';
      const providerHappUrl =
        dirname(fileURLToPath(import.meta.url)) +
        '/../../providers/fcm/apps/notifications_provider_fcm/workdir/notifications_provider_fcm.happ';
      const recipientHappUrl =
        dirname(fileURLToPath(import.meta.url)) +
        '/../../providers/fcm/apps/notifications_provider_fcm_recipient/workdir/notifications_fcm_recipient.happ';

      // Add 2 players with the test hApp to the Scenario. The returned players
      // can be destructured.
      const [fixture, provider, recipient] = await scenario.addPlayersWithApps([
        { appBundleSource: { path: testHappUrl } },
        { appBundleSource: { path: providerHappUrl } },
        { appBundleSource: { path: recipientHappUrl } },
      ]);

      const providerNotifications = provider.cells.find(
        c => !c.name.includes('fcm')
      );
      const providerFCMProvider = provider.cells.find(c =>
        c.name.includes('fcm')
      );
      const recipientCell = recipient.namedCells.get('notifications');
      assert.equal(
        providerNotifications.cell_id[0].toString(),
        recipientCell.cell_id[0].toString()
      );
      assert.equal(
        fixture.cells[0].cell_id[0].toString(),
        recipientCell.cell_id[0].toString()
      );
      assert.equal(
        fixture.cells[0].cell_id[0].toString(),
        providerNotifications.cell_id[0].toString()
      );

      // Shortcut peer discovery through gossip and register all agents in every
      // conductor of the scenario.
      await scenario.shareAllAgents();

      /* Setup provider */

      // Publish Service Account Key
      await providerFCMProvider.callZome({
        zome_name: 'notifications_provider_fcm',
        fn_name: 'publish_new_service_account_key',
        payload: {
          /// private_key
          private_key: 'pk',
          /// client_email
          client_email: 'pk',
          /// token_uri
          token_uri: 'tu',
        },
      });
      // assert_eq!(record_1, None);

      // Announce as provider
      await providerNotifications.callZome({
        zome_name: 'notifications',
        fn_name: 'announce_as_provider',
        payload: null,
      });

      await dhtSync(
        [provider, recipient, fixture],
        recipient.cells[0].cell_id[0]
      );

      /* Setup recipient */
      // Register FCM token
      await recipient.cells[0].callZome({
        zome_name: 'notifications_provider_fcm_recipient',
        fn_name: 'register_new_fcm_token',
        payload: 'NEW_FCM_TOKEN',
      });

      // Shutdown recipient

      /* Send notification */

      await Promise.race([
        new Promise((_, reject) => setTimeout(() => reject(), 20000)),
        new Promise(async resolve => {
          // FCM provider zome sends signal
          (provider.appAgentWs as AppAgentWebsocket).on('signal', signal => {
            console.log(signal);
            resolve(undefined);
          });
          // Send notification from fixture notification zome
          fixture.cells[0].callZome({
            zome_name: 'notifications',
            fn_name: 'request_notify_agent',
            payload: {
              notification: encode({
                message: 'sometext',
              }),
              agent: recipient.agentPubKey,
            },
          });
        }),
      ]);
    },
    true,
    { timeout: 120000 }
  );
});
