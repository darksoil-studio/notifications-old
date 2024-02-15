import { test, assert } from 'vitest';
import { dirname } from 'path';
import { fileURLToPath } from 'url';

import { runScenario, dhtSync, } from '@holochain/tryorama';
import { AppAgentWebsocket } from '@holochain/client';

test('setup provider, sender and recipient, and send an email', async t => {
  await runScenario(
    async scenario => {
      const testHappUrl =
        dirname(fileURLToPath(import.meta.url)) +
        '/../fixture/workdir/test_app.happ';
      const providerHappUrl =
        dirname(fileURLToPath(import.meta.url)) +
        '/../../provider/app/email_notifications_provider.happ';

      // Add 2 players with the test hApp to the Scenario. The returned players
      // can be destructured.
      const [provider, sender, recipient] = await scenario.addPlayersWithApps([
        { appBundleSource: { path: providerHappUrl } },
        { appBundleSource: { path: testHappUrl } },
        { appBundleSource: { path: testHappUrl } },
      ]);

      const providerServiceCell = provider.namedCells.get('email_notifications_service');
      const providerProviderCell = provider.namedCells.get('email_notifications_provider')
      assert.equal(
        sender.cells[0].cell_id[0].toString(),
        providerServiceCell.cell_id[0].toString()
      );

      // Shortcut peer discovery through gossip and register all agents in every
      // conductor of the scenario.
      await scenario.shareAllAgents();

      /* Setup provider */

      // Publish Email Credentials
      const emailCredentials = {
        username: 'some@address.com',
        password: 'some@address.com',
        smtp_relay_url: 'smtp.gmail.com'
      };
      await providerProviderCell.callZome({
        zome_name: 'email_notifications_provider',
        fn_name: 'publish_new_email_credentials',
        payload: emailCredentials,
      });

      await dhtSync(
        [provider, sender],
        sender.cells[0].cell_id[0]
      );

      const emailAddress = 'some@address.com';
      // Register email address
      await recipient.cells[0].callZome({
        zome_name: 'email_notifications_service',
        fn_name: 'register_email_address',
        payload: emailAddress,
      });

      /* Send email notification */

      const email = {
        subject: 'Some important email message',
        body: 'Lorem ipsum blabla',
      };

      await Promise.race([
        new Promise((_, reject) => setTimeout(() => reject(), 20000)),
        new Promise(async resolve => {
          // FCM provider zome sends signal
          (provider.appAgentWs as AppAgentWebsocket).on('signal', signal => {
            console.log(signal);
            const payload = signal.payload as any;
            assert.deepEqual(payload.email_address, emailAddress)
            assert.deepEqual(payload.email, email)
            assert.deepEqual(payload.credentials, emailCredentials)
            resolve(undefined);
          });
          // Send notification from fixture notification zome
          sender.cells[0].callZome({
            zome_name: 'email_notifications_service',
            fn_name: 'request_send_email',
            payload: {
              email,
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
