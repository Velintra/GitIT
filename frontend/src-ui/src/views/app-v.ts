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
  <header>
    <app-header></app-header>
  </header>
  <main>
    <p>Welcome</p>
  </main>
`;

@customElement("app-v")
export class AppView extends BaseViewElement {
  #mainEl!: HTMLElement;

  init() {
    const content = document.importNode(HTML, true);
    this.#mainEl = getFirst(content, "main");
    this.replaceChildren(content);
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "app-v": AppView;
  }
}
