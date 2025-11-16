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

const HTML = html`
  <main>
    <setup-v></setup-v>
  </main>
`;

@customElement("root-v")
export class RootView extends BaseViewElement {
  #mainEl!: HTMLElement;

  init() {
    const content = document.importNode(HTML, true);
    this.#mainEl = getFirst(content, "main");
    this.replaceChildren(content);
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "root-v": RootView;
  }
}
