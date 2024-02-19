import { css, html, LitElement } from 'lit';
import { provide } from '@lit/context';
import { customElement, property } from 'lit/decorators.js';

import { notificationsProviderStoreContext } from '../context.js';
import { NotificationsProviderStore } from '../notifications-provider-store.js';

@customElement('notifications-provider-context')
export class NotificationsProviderContext extends LitElement {
  @provide({ context: notificationsProviderStoreContext })
  @property({ type: Object })
  store!: NotificationsProviderStore;

  render() {
    return html`<slot></slot>`;
  }

  static styles = css`
    :host {
      display: contents;
    }
  `;
}
