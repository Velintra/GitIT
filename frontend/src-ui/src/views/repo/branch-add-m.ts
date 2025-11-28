import {
  BaseHTMLElement,
  customElement,
  elem,
  getFirst,
  trigger,
  html,
  onDoc,
  first,
  onEvent,
  OnEvent,
} from "dom-native";

import CDSTextInput from "@carbon/web-components/es/components/text-input/text-input";
import CDSSelect from "@carbon/web-components/es/components/select/select";
import CDSTextarea from "@carbon/web-components/es/components/textarea/textarea";
import CDSModal from "@carbon/web-components/es/components/modal/modal";
import "@carbon/web-components/es/components/form/index.js";
import "@carbon/web-components/es/components/text-input/index.js";
import "@carbon/web-components/es/components/modal/index.js";
import "@carbon/web-components/es/components/button/index.js";
import "@carbon/web-components/es/components/textarea/index.js";

const HTML = html`
  <cds-modal open class="add-branch">
    <cds-modal-header>
      <cds-modal-close-button></cds-modal-close-button>
      <cds-modal-label>Branch</cds-modal-label>
      <cds-modal-heading>Create a branch</cds-modal-heading>
    </cds-modal-header>
    <cds-modal-body>
      <cds-form-item>
        <cds-text-input placeholder="e.g. dev-branch" label="Branch name">
        </cds-text-input>
      </cds-form-item>
      <div class="message"></div>
    </cds-modal-body>
    <cds-modal-footer>
      <cds-modal-footer-button
        class="cancel"
        kind="secondary"
        data-modal-close=""
        >Cancel</cds-modal-footer-button
      >
      <cds-modal-footer-button class="add">Add</cds-modal-footer-button>
    </cds-modal-footer>
  </cds-modal>
`;

@customElement("branch-add-m")
class BranchAddModal extends BaseHTMLElement {
  #branchNameInputEl!: CDSTextInput;

  private get footerMessage(): HTMLElement {
    return first(this, "cds-modal > cds-modal-body .message")!;
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

  @onEvent(
    "pointerup",
    ".add-branch > cds-modal-header > cds-modal-close-button, .add-branch > cds-modal-footer > .cancel",
  )
  onAddClose() {
    this.remove();
  }

  @onEvent("pointerup", "cds-modal")
  onBackdropClick(evt: PointerEvent) {
    const target = evt.target as HTMLElement;

    if (target.tagName === "CDS-MODAL") {
      this.remove();
    }
  }

  @onEvent("pointerup", ".add")
  onAddClick(evt: OnEvent) {
    const name = this.#branchNameInputEl.value.trim() as string;

    trigger(this, "BRANCH-ADD", {
      detail: {
        name,
      },
    });

    this.remove();
  }

  init() {
    super.init();

    const content = document.importNode(HTML, true);
    this.#branchNameInputEl = getFirst(
      content,
      "cds-text-input",
    ) as CDSTextInput;

    this.replaceChildren(content);
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "branch-add-modal": BranchAddModal;
  }
}
