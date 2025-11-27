import {
  BaseHTMLElement,
  customElement,
  elem,
  first,
  getFirst,
  html,
  onEvent,
  onHub,
} from "dom-native";
import { BaseViewElement } from "../base-v";
import { open } from "@tauri-apps/plugin-dialog";
import { repoFmc } from "../model";
import { AppSwitcherIco, SettingsIco } from "../icos";
import { CDSHeaderName } from "@carbon/web-components";
import "@carbon/web-components/es/components/ui-shell/index.js";
import "@carbon/web-components/es/components/tile/index.js";
import { pathAt } from "../router";
import { isNotEmpty } from "utils-min";

const tagNameByPath: { [name: string]: string } = {
  branches: "branch-v",
};

const HTML = html`
  <header>
      <cds-header aria-label="GitIT">
        <cds-header-name id="repo-name" href="javascript:void 0" prefix="GitIT"
          ></cds-header-name
        >
        <div class="cds--header__global">
            <cds-header-global-action
               aria-label="App Switcher"
               class="app-switcher"
               panel-id="switcher-panel"
             >
                 ${AppSwitcherIco}
             </cds-header-global-action>
             <cds-header-global-action
               aria-label="App Switcher"
               class="app-switcher"
               panel-id="user-panel"
             >
                 ${SettingsIco}
             </cds-header-global-action>
          <cds-header-panel class="user-panel" id="user-panel" aria-label="Header Panel">
            <cds-switcher aria-label="Switcher Container">
              <cds-switcher-item class="vault-settings" aria-label="Settings"
                >Vault Settings</cds-switcher-item
              >
            </cds-switcher>
          <cds-header-panel class="repo-panel" id="switcher-panel" aria-label="Repo Panel">
            <cds-switcher aria-label="Switcher Container">
                <cds-switcher-item class="overview" aria-label="Overview"
                  >Overview</cds-switcher-item
                >
              <cds-switcher-item class="commits" aria-label="Commits"
                >Commits</cds-switcher-item
              >
              <cds-switcher-item class="branches" aria-label="Branches" view="/branches"
                >Branches</cds-switcher-item
              >
              <cds-switcher-item class="tags" aria-label="Tags"
                >Tags</cds-switcher-item
              >

            </cds-switcher>
          </cds-header-panel>
        </div>
      </cds-header>
  </header>
  <main>
    <p id="welcome">Welcome please click here to select a github repository</p>
  </main>
`;

@customElement("app-v")
export class AppView extends BaseViewElement {
  #mainEl!: HTMLElement;
  #headerEl!: CDSHeaderName;

  @onHub("Route", "CHANGE")
  routeChange() {
    this.refresh();
  }

  refresh() {
    if (this.hasPathChanged(0)) {
      const newPath = pathAt(0);
      const name = isNotEmpty(newPath) ? newPath : "";

      const tagName = tagNameByPath[name];

      if (tagName) {
        this.#mainEl.replaceChildren(elem(tagName));
      } else {
        const p = elem("p", {
          id: "welcome",
          $: {
            textContent:
              "Welcome please click here to select a github repository",
          },
        });

        this.#mainEl.replaceChildren(p);
      }

      // if (name === "app") {
      //   this.#headerEl.innerHTML = "<lf-app-header></lf-app-header>";
      // } else {
      //   this.#headerEl.innerHTML = "<lf-landing-header></lf-landing-header>";
      // }
    }
  }

  @onEvent("pointerup", "#welcome")
  async onWelcomeClick() {
    const file = await open({
      multiple: false,
      directory: true,
      title: "GitIT",
    });
    if (file !== null) {
      const result = await repoFmc.open(file);
      this.#headerEl.prefix = result;
    }
  }

  init() {
    const content = document.importNode(HTML, true);
    [this.#mainEl, this.#headerEl] = getFirst(
      content,
      "main",
      "#repo-name",
    ) as [HTMLElement, CDSHeaderName];
    this.replaceChildren(content);
    this.refresh();
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "app-v": AppView;
  }
}
