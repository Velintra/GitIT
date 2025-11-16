import {
  BaseHTMLElement,
  customElement,
  getFirst,
  html,
  onEvent,
} from "dom-native";

const HTML = html`
  <cds-modal size="sm" open>
    <cds-modal-header>
      <cds-modal-close-button></cds-modal-close-button>
      <cds-modal-heading>An Error Occurred</cds-modal-heading>
    </cds-modal-header>
    <cds-modal-body></cds-modal-body>
  </cds-modal>
`;

@customElement("dg-dialog-error")
export class DgDialogError extends BaseHTMLElement {
  #messageEl!: HTMLElement;
  #msg!: string;

  set msg(m: string) {
    this.#msg = m;
  }

  setEl() {
    if (this.#messageEl) {
      this.#messageEl.textContent = this.#msg;
    }
  }

  @onEvent("pointerup", "cds-modal-close-button")
  doOk() {
    this.remove();
  }

  init() {
    super.init();
    const content = document.importNode(HTML, true);

    this.#messageEl = getFirst(content, "cds-modal-body") as HTMLElement;

    this.replaceChildren(content);
    this.setEl();
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "dg-dialog-error": DgDialogError;
  }
}
