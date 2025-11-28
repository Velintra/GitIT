import {
  BaseHTMLElement,
  cherryChild,
  customElement,
  elem,
  first,
  frag,
  getFirst,
  html,
  onEvent,
  onHub,
} from "dom-native";
import { BaseViewElement } from "../../base-v";
import { repoFmc } from "../../model";
import { CDSTag } from "@carbon/web-components";
import "@carbon/web-components/es/components/ui-shell/index.js";
import "@carbon/web-components/es/components/tile/index.js";
import { Branch } from "src-ui/src/bindings";
import { TAG_TYPE } from "@carbon/web-components/es/components/tag/defs";
import "@carbon/web-components/es/components/tag/index.js";

const HTML = html` <section></section> `;

@customElement("branch-v")
export class BranchView extends BaseViewElement {
  #contentEl!: HTMLElement;

  @onHub("Model", "branch", "create, delete")
  onChange() {
    this.refreshContent();
  }

  async refreshContent(first_refresh?: boolean) {
    const branches = await repoFmc.list_branches();

    const content = frag(branches, (b: Branch) => {
      let div = elem("branch-tile", { $: { branch: b } });

      return div;
    });

    this.#contentEl.replaceChildren(content);
  }

  init() {
    const content = document.importNode(HTML, true);
    this.#contentEl = getFirst(content, "section");
    this.replaceChildren(content);
    this.refreshContent(true);
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "branch-v": BranchView;
  }
}

const BRANCH_TILE_HTML = html`
  <cds-tile class="branch-tile">
    <span class="name"></span>
    <cds-tag class="type" size="md" title="Branch type"></cds-tag>
    <span class="id"></span>
  </cds-tile>
`;

@customElement("branch-tile")
export class BranchTile extends BaseHTMLElement {
  #branch!: Branch;

  #nameEl!: HTMLElement;
  #typeEL!: CDSTag;
  #idEl!: HTMLElement;

  set branch(b: Branch) {
    this.#branch = b;
  }
  get task() {
    return this.#branch;
  }

  setEls() {
    if (this.#branch && this.#nameEl && this.#typeEL && this.#idEl) {
      this.#nameEl.textContent = this.#branch.name;
      this.#typeEL.textContent = this.#branch.kind;
      this.#typeEL.type =
        this.#branch.kind === "Local"
          ? ("green" as TAG_TYPE)
          : ("blue" as TAG_TYPE);
      this.#idEl.textContent = this.#branch.target;
    }
  }

  init() {
    super.init();
    const content = document.importNode(BRANCH_TILE_HTML, true);
    [this.#nameEl, this.#typeEL, this.#idEl] = getFirst(
      content,
      ".name",
      ".type",
      ".id",
    ) as [HTMLElement, CDSTag, HTMLElement];
    this.replaceChildren(content);
    this.setEls();
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "branch-tile": BranchTile;
  }
}
