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
  trigger,
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
      type="password"
      show-password-visibility-toggle=""
      class="vault-pwd"
      label="Set your vault password"
    ></cds-text-input>

    <cds-button class="set">Set vault password</cds-button>
  </cds-stack>
  <footer>
    <div class="message"></div>
  </footer>
`;

@customElement("stage-vault")
export class StageVault extends BaseViewElement {
  #vaultPwdInputEl!: CDSTextInput;

  @onEvent("click", ".set")
  async onSave() {
    const password = this.#vaultPwdInputEl.value.trim() as string;

    if (password === "") {
      return;
    }
    await ipc_invoke("init_vault", "params", { password });
  }

  init() {
    const content = document.importNode(HTML, true);
    this.#vaultPwdInputEl = getFirst(
      content,
      "cds-text-input.vault-pwd",
    ) as CDSTextInput;
    this.replaceChildren(content);
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "stage-vault": StageVault;
  }
}
