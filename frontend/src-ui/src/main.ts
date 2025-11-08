import { html, trigger } from "dom-native";
import "./event.js";
import "./views/index.js";

document.addEventListener("DOMContentLoaded", async function (event) {
  trigger(this, "APP_LOADED");
  console.log("LOADED");
});
