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
import { open } from "@tauri-apps/plugin-dialog";
import { repoFmc } from "../model";

const HTML = html`
  <header>
    <app-header></app-header>
  </header>
  <main>
    <p id="welcome">Welcome please click here to select a github repository</p>
  </main>
`;

@customElement("app-v")
export class AppView extends BaseViewElement {
  #mainEl!: HTMLElement;

  @onEvent("pointerup", "#welcome")
  async onWelcomeClick() {
    const file = await open({
      multiple: false,
      directory: true,
      title: "GitIT",
    });
    if (file !== null) {
      const result = await repoFmc.open(file);
      console.log(result);
    }
  }

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
