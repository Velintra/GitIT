import {
  BaseHTMLElement,
  cherryChild,
  customElement,
  elem,
  first,
  frag,
  getFirst,
  html,
  on,
  OnEvent,
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
import "@carbon/web-components/es/components/button/index.js";
import "@carbon/web-components/es/components/overflow-menu/index.js";

const HTML = html`
  <div class="top-bar">
    <cds-button class="add-btn">Create a Branch</cds-button>
  </div>
  <section></section>
`;

@customElement("branch-v")
export class BranchView extends BaseViewElement {
  #contentEl!: HTMLElement;

  @onHub("Model", "branch", "create, delete")
  onChange() {
    this.refreshContent();
  }

  @onEvent("click", ".add-btn")
  onCreateClick() {
    const el = elem("branch-add-m");
    this.appendChild(el);
    on(el, "BRANCH-ADD", async (evt: OnEvent<{ name: string }>) => {
      try {
        const id = await repoFmc.create_branch(evt.detail.name);
        console.log(id);
      } catch (ex) {
        console.log(ex);
      }
    });
  }

  @onEvent("cds-overflow-menu-item-clicked")
  async onItemClick(evt: OnEvent) {
    if (evt.target.className === "delete") {
      const row = evt.target.closest("branch-tile")!;
      repoFmc.delete_branch(row.getAttribute("branch-name") as string);
    }
  }

  async refreshContent(first_refresh?: boolean) {
    const branches = await repoFmc.list_branches();

    const content = frag(branches, (b: Branch) => {
      let div = elem("branch-tile", {
        "branch-name": b.name,
        $: { branch: b },
      });

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
  <cds-tile class="branch-tile" data-floating-menu-container>
    <span class="name"></span>
    <cds-tag class="type" size="md" title="Branch type"></cds-tag>
    <span class="id"></span>
    <cds-overflow-menu size="md" index="1">
      <svg
        focusable="false"
        preserveAspectRatio="xMidYMid meet"
        xmlns="http://www.w3.org/2000/svg"
        fill="currentColor"
        class="cds--overflow-menu__icon"
        slot="icon"
        width="16"
        height="16"
        viewBox="0 0 32 32"
        aria-hidden="true"
      >
        <circle cx="16" cy="8" r="2"></circle>
        <circle cx="16" cy="16" r="2"></circle>
        <circle cx="16" cy="24" r="2"></circle>
      </svg>
      <span slot="tooltip-content"> Options </span>
      <cds-overflow-menu-body flipped>
        <cds-overflow-menu-item>Rename</cds-overflow-menu-item>
        <cds-overflow-menu-item>Merge</cds-overflow-menu-item>
        <cds-overflow-menu-item divider danger class="delete"
          >Delete</cds-overflow-menu-item
        >
      </cds-overflow-menu-body>
    </cds-overflow-menu>
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
