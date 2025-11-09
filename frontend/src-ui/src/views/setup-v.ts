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
import { BaseViewElement } from "../base-v";
import "@carbon/web-components/es/components/button/button.js";
import "@carbon/web-components/es/components/text-input/index.js";
import "@carbon/web-components/es/components/stack/stack.js";
import { CDSTextInput } from "@carbon/web-components";
import { ipc_invoke } from "../ipc";
import { appDataDir } from "@tauri-apps/api/path";

const HTML = html`
  <main>
    <h1>Welcome to GitIT</h1>
    <p>
      Enter your username and password. They will be securely stored in the
      stronghold
    </p>
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

      <cds-text-input
        required=""
        type="password"
        show-password-visibility-toggle=""
        class="vault-pwd"
        label="Set your vault password"
      ></cds-text-input>

      <cds-button class="save">Save</cds-button>
    </cds-stack>
  </main>
  <footer>
    <div class="message"></div>
  </footer>
`;

@customElement("setup-v")
export class SetupView extends BaseViewElement {
  #pwdInputEl!: CDSTextInput;
  #vaultPwdInputEl!: CDSTextInput;

  #usernameInputEl!: CDSTextInput;

  private get footerMessage(): HTMLElement {
    return first(this, "footer .message")!;
  }

  private set message(txt: string | null) {
    if (txt != null) {
      this.footerMessage.textContent = txt;
      this.classList.add("has-message");
    } else {
      if (this.classList.contains("has-message")) {
        this.footerMessage.textContent = "";
        this.classList.remove("has-message");
      }
    }
  }

  @onHub("Handler", "creds", "save")
  async onCredsEvt() {
    const res = await ipc_invoke("save_credentials", "params");
    console.log(res);
  }

  @onEvent("click", ".save")
  async onSave() {
    const username = this.#usernameInputEl.value.trim() as string;
    const password = this.#pwdInputEl.value.trim() as string;
    const vaultPwd = this.#vaultPwdInputEl.value.trim() as string;

    if (username === "" || password === "" || vaultPwd === "") {
      this.message = "All fields are required";
      return;
    }

    // todo call the ipc handler
  }

  init() {
    const content = document.importNode(HTML, true);
    [this.#usernameInputEl, this.#pwdInputEl, this.#vaultPwdInputEl] = getFirst(
      content,
      "cds-text-input.username",
      "cds-text-input.pwd",
      "cds-text-input.vault-pwd",
    ) as [CDSTextInput, CDSTextInput, CDSTextInput];
    this.replaceChildren(content);
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "setup-v": SetupView;
  }
}
