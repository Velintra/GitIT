import { invoke } from "@tauri-apps/api/core";
import { deepFreeze } from "utils-min";
import { trigger, getFirst, first } from "dom-native";

export async function ipc_invoke(
  method: string,
  params?: object,
): Promise<any> {
  const response: any = await invoke(method, { rpcReq: params });

  if (response.error != null) {
    console.log("ERROR - ipc_invoke - ipc_invoke error", response);
    if (first("app-v")) {
      trigger(getFirst("app-v"), "IPC_ORG_ERROR", {
        detail: response.error,
      });
    }
    throw new Error(response.error);
  } else {
    return deepFreeze(response.result);
  }
}
