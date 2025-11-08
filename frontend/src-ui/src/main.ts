import { html, trigger } from "dom-native";
import "./event.js";
import { ipc_invoke } from "./ipc.js";

document.addEventListener("DOMContentLoaded", async function (event) {
  trigger(this, "APP_LOADED");
  const rpc_req = {
    jsonrpc: "2.0",
    id: 1,
    method: "open_repo",
    params: {
      path: "/home/pc/Desktop/Arek2/ProgrammingProjects/ZeroFlux/GitIT",
    },
  };

  try {
    const result = await ipc_invoke("rpc_handler", rpc_req).then(
      (res) => res.data,
    );
    console.log("RPC result:", result);
  } catch (err) {
    console.error("IPC error:", err);
  }

  const rpc_req2 = {
    jsonrpc: "2.0",
    id: 1,
    method: "list_branches",
  };

  try {
    const result = await ipc_invoke("rpc_handler", rpc_req2).then(
      (res) => res.data,
    );
    console.log("RPC result:", result);
  } catch (err) {
    console.error("IPC error:", err);
  }
});
