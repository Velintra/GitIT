import {
  BaseHTMLElement,
  customElement,
  elem,
  first,
  getFirst,
  html,
  onEvent,
  onHub,
  pull,
} from "dom-native";
import { BaseViewElement } from "../../base-v";
import "@carbon/web-components/es/components/button/button.js";
import "@carbon/web-components/es/components/text-input/index.js";
import "@carbon/web-components/es/components/stack/stack.js";
import { CDSTextInput } from "@carbon/web-components";
import { ipc_invoke } from "../../ipc";
import { appDataDir } from "@tauri-apps/api/path";

const HTML = html`
  <cds-stack gap="7">
    <cds-text-input
      required=""
      label="Enter your username"
      class="username"
      enable-counter
      max-count="50"
    ></cds-text-input>

    <cds-text-input
      required=""
      type="password"
      show-password-visibility-toggle=""
      class="pwd"
      label="Enter your password"
    ></cds-text-input>

    <cds-button class="save">Save</cds-button>
  </cds-stack>
`;

@customElement("stage-creds")
export class StageCreds extends BaseViewElement {
  #usernameInputEl!: CDSTextInput;
  #pwdInputEl!: CDSTextInput;

  @onEvent("click", ".save")
  async onSave() {
    const password = this.#pwdInputEl.value.trim() as string;
    const username = this.#usernameInputEl.value.trim() as string;

    if (password === "" || username === "") {
      return;
    }

    await ipc_invoke("save_credentials", "params", { username, password });
  }

  init() {
    const content = document.importNode(HTML, true);
    [this.#usernameInputEl, this.#pwdInputEl] = getFirst(
      content,
      "cds-text-input.username",
      "cds-text-input.pwd",
    ) as [CDSTextInput, CDSTextInput];

    this.replaceChildren(content);
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "stage-creds": StageCreds;
  }
}
