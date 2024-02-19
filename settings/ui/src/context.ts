import { createContext } from '@lit/context';
import { NotificationsProviderStore } from './notifications-provider-store.js';

export const notificationsProviderStoreContext = createContext<NotificationsProviderStore>(
  'hc_zome_notifications_provider/store'
);

