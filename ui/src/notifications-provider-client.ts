import { 
  SignedActionHashed,
  CreateLink,
  Link,
  DeleteLink,
  Delete,
  AppAgentClient, 
  Record, 
  ActionHash, 
  EntryHash, 
  AgentPubKey,
} from '@holochain/client';
import { isSignalFromCellWithRole, EntryRecord, ZomeClient } from '@holochain-open-dev/utils';

import { NotificationsProviderSignal } from './types.js';

export class NotificationsProviderClient extends ZomeClient<NotificationsProviderSignal> {

  constructor(public client: AppAgentClient, public roleName: string, public zomeName = 'notifications_provider') {
    super(client, roleName, zomeName);
  }
}
