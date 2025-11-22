import {
  all,
  BaseHTMLElement,
  customElement,
  getFirst,
  html,
  OnEvent,
  onEvent,
  first,
  onHub,
} from "dom-native";

import "@carbon/web-components/es/components/ui-shell/index.js";
import "@carbon/web-components/es/components/modal/index.js";
import "@carbon/web-components/es/components/modal/defs.js";
import "@carbon/web-components/es/components/text-input/index.js";
import "@carbon/web-components/es/components/content-switcher/index.js";
import CDSContentSwitcher from "@carbon/web-components/es/components/content-switcher/content-switcher";

const HTML = html`
  <cds-header-global-action
    aria-label="App Switcher"
    class="app-switcher"
    panel-id="switcher-panel"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 32 32"
      fill="currentColor"
      width="32"
      height="32"
      preserveAspectRatio="xMidYMid meet"
      aria-hidden="true"
      style="display: block; margin: auto;"
      class="cds--header_icon"
    >
      <path
        d="M14 4H18V8H14zM4 4H8V8H4zM24 4H28V8H24zM14 14H18V18H14zM4 14H8V18H4zM24 14H28V18H24zM14 24H18V28H14zM4 24H8V28H4zM24 24H28V28H24z"
      />
    </svg>
  </cds-header-global-action>
  <cds-header-global-action
    aria-label="App Switcher"
    class="app-switcher"
    panel-id="user-panel"
  >
    <svg
      id="icon"
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 32 32"
      fill="currentColor"
      width="32"
      height="32"
      preserveAspectRatio="xMidYMid meet"
      aria-hidden="true"
      style="display: block; margin: auto;"
      class="cds--header_icon"
    >
      <path
        d="M16,4a5,5,0,1,1-5,5,5,5,0,0,1,5-5m0-2a7,7,0,1,0,7,7A7,7,0,0,0,16,2Z"
      />
      <path
        d="M26,30H24V25a5,5,0,0,0-5-5H13a5,5,0,0,0-5,5v5H6V25a7,7,0,0,1,7-7h6a7,7,0,0,1,7,7Z"
      />
    </svg>
  </cds-header-global-action>
`;

@customElement("header-action-v")
export class HeaderActionView extends BaseHTMLElement {
  init() {
    const content = document.importNode(HTML, true);
    this.replaceChildren(content);
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "header-action-v": HeaderActionView;
  }
}
