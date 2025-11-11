import { invoke } from "@tauri-apps/api/core";
import { deepFreeze } from "utils-min";
import { trigger, getFirst, first } from "dom-native";

// export async function ipc_invoke(
//   method: string,
//   key: string = "rpcReq",
//   params?: object,
// ): Promise<any> {
//   const args = params !== undefined ? { [key]: params } : undefined;
//   const response: any = await invoke(method, args);

//   if (response.error != null) {
//     console.log("ERROR - ipc_invoke - ipc_invoke error", response);
//     if (first("root-v")) {
//       trigger(getFirst("root-v"), "IPC_ERROR", {
//         detail: response.error,
//       });
//     }
//     throw new Error(response.error);
//   } else {
//     return deepFreeze(response.result);
//   }
// }

export async function ipc_invoke(
  method: string,
  key: string = "rpcReq",
  params?: object,
): Promise<any> {
  const args = params !== undefined ? { [key]: params } : undefined;

  try {
    const response: any = await invoke(method, args);

    return deepFreeze(response);
  } catch (err: any) {
    console.error("ipc_invoke failed", err);
    if (first("root-v")) {
      trigger(getFirst("root-v"), "IPC_ERROR", { detail: err });
    }
    throw err;
  }
}
