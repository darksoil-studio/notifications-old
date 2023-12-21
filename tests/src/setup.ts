import { 
  AgentPubKey,
  EntryHash,
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash,
  fakeDnaHash,
  AppAgentCallZomeRequest,
  AppAgentWebsocket,
  encodeHashToBase64 
} from '@holochain/client';
import { encode } from '@msgpack/msgpack';
import { Scenario } from '@holochain/tryorama';
import { EntryRecord } from '@holochain-open-dev/utils';
import { dirname } from 'path';
import { fileURLToPath } from 'url';
import { NotificationsProviderClient } from '../../ui/src/notifications-provider-client.js';
import { NotificationsProviderStore } from '../../ui/src/notifications-provider-store.js';

export async function setup(scenario: Scenario) {
  const testHappUrl =
    dirname(fileURLToPath(import.meta.url)) + '/../../workdir/notifications_provider.happ';

  // Add 2 players with the test hApp to the Scenario. The returned players
  // can be destructured.
  const [alice, bob] = await scenario.addPlayersWithApps([
    { appBundleSource: { path: testHappUrl } },
    { appBundleSource: { path: testHappUrl } },
  ]);

  // Shortcut peer discovery through gossip and register all agents in every
  // conductor of the scenario.
  await scenario.shareAllAgents();

  const aliceStore = new NotificationsProviderStore(
    new NotificationsProviderClient(alice.appAgentWs as any, 'notifications_provider_test', 'notifications_provider')
  );

  const bobStore = new NotificationsProviderStore(
    new NotificationsProviderClient(bob.appAgentWs as any, 'notifications_provider_test', 'notifications_provider')
  );

  // Shortcut peer discovery through gossip and register all agents in every
  // conductor of the scenario.
  await scenario.shareAllAgents();

  return {
    alice: {
      player: alice,
      store: aliceStore,
    },
    bob: {
      player: bob,
      store: bobStore,
    },
  };
}

