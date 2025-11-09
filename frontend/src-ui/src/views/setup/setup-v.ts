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
  <main>
    <h1>Welcome to GitIT</h1>
    <p>
      Enter your git username and password. They will be securely stored in the
      stronghold
    </p>
    <section class="content">
      <stage-vault></stage-vault>
    </section>
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
  #contentEl!: HTMLElement;

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
  async onCredsSave() {
    const res = await ipc_invoke("get_credentials", "params");
  }

  @onHub("Handler", "vault", "init")
  onVaultInit() {
    this.#contentEl.replaceChildren(elem("stage-creds"));
  }

  init() {
    const content = document.importNode(HTML, true);
    this.#contentEl = getFirst(content, "section.content");
    this.replaceChildren(content);
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "setup-v": SetupView;
  }
}
