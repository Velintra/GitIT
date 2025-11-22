import { BaseHTMLElement, customElement, html } from "dom-native";
import "@carbon/web-components/es/components/ui-shell/index.js";
import "@carbon/web-components/es/components/content-switcher/index.js";
import { BaseViewElement } from "../base-v";

const HTML = html`
    <cds-header aria-label="GitIT">
      <cds-header-name href="javascript:void 0" prefix="GIT"
        >[IT]</cds-header-name
      >
      <div class="cds--header__global">
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
               <svg id="icon"
                   xmlns="http://www.w3.org/2000/svg"
                   viewBox="0 0 32 32"
                   fill="currentColor"
                   width="32" height="32"
                   preserveAspectRatio="xMidYMid meet"
                   aria-hidden="true" style="display: block; margin: auto;"
                   class="cds--header_icon"
               >
                   <path d="M27,16.76c0-.25,0-.5,0-.76s0-.51,0-.77l1.92-1.68A2,2,0,0,0,29.3,11L26.94,7a2,2,0,0,0-1.73-1,2,2,0,0,0-.64.1l-2.43.82a11.35,11.35,0,0,0-1.31-.75l-.51-2.52a2,2,0,0,0-2-1.61H13.64a2,2,0,0,0-2,1.61l-.51,2.52a11.48,11.48,0,0,0-1.32.75L7.43,6.06A2,2,0,0,0,6.79,6,2,2,0,0,0,5.06,7L2.7,11a2,2,0,0,0,.41,2.51L5,15.24c0,.25,0,.5,0,.76s0,.51,0,.77L3.11,18.45A2,2,0,0,0,2.7,21L5.06,25a2,2,0,0,0,1.73,1,2,2,0,0,0,.64-.1l2.43-.82a11.35,11.35,0,0,0,1.31.75l.51,2.52a2,2,0,0,0,2,1.61h4.72a2,2,0,0,0,2-1.61l.51-2.52a11.48,11.48,0,0,0,1.32-.75l2.42.82a2,2,0,0,0,.64.1,2,2,0,0,0,1.73-1L29.3,21a2,2,0,0,0-.41-2.51ZM25.21,24l-3.43-1.16a8.86,8.86,0,0,1-2.71,1.57L18.36,28H13.64l-.71-3.55a9.36,9.36,0,0,1-2.7-1.57L6.79,24,4.43,20l2.72-2.4a8.9,8.9,0,0,1,0-3.13L4.43,12,6.79,8l3.43,1.16a8.86,8.86,0,0,1,2.71-1.57L13.64,4h4.72l.71,3.55a9.36,9.36,0,0,1,2.7,1.57L25.21,8,27.57,12l-2.72,2.4a8.9,8.9,0,0,1,0,3.13L27.57,20Z" transform="translate(0 0)"/><path d="M16,22a6,6,0,1,1,6-6A5.94,5.94,0,0,1,16,22Zm0-10a3.91,3.91,0,0,0-4,4,3.91,3.91,0,0,0,4,4,3.91,3.91,0,0,0,4-4A3.91,3.91,0,0,0,16,12Z" transform="translate(0 0)"/>
               </svg>
           </cds-header-global-action>
        <cds-header-panel class="user-panel" id="user-panel" aria-label="Header Panel">
          <cds-switcher aria-label="Switcher Container">
            <cds-switcher-item class="vault-settings" aria-label="Settings"
              >Vault Settings</cds-switcher-item
            >
          </cds-switcher>
        <cds-header-panel class="repo-panel" id="switcher-panel" aria-label="Repo Panel">
          <cds-switcher aria-label="Switcher Container">
            <cds-switcher-item class="commits" aria-label="Commits"
              >Commits</cds-switcher-item
            >
            <cds-switcher-item class="branches" aria-label="Branches"
              >Branches</cds-switcher-item
            >
            <cds-switcher-item class="tags" aria-label="Tags"
              >Tags</cds-switcher-item
            >

          </cds-switcher>
        </cds-header-panel>
      </div>
    </cds-header>
`;

@customElement("app-header")
export class AppHeader extends BaseViewElement {
  init() {
    let content = document.importNode(HTML, true);
    this.replaceChildren(content);
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "app-header": AppHeader;
  }
}
