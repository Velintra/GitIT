import { Branch } from "../bindings";
import { ipc_invoke } from "../ipc";
import { makeRpcRequest, RpcSuccess } from "../rpc-utils";

class BaseFmc {
  #cmd_suffix: string;

  get cmd_suffix() {
    return this.#cmd_suffix;
  }

  constructor(cmd_suffix: string) {
    this.#cmd_suffix = cmd_suffix;
  }
}

class RepoFmc extends BaseFmc {
  constructor() {
    super("repo");
  }

  async list_branches(): Promise<Branch[]> {
    const req = makeRpcRequest(`list_branches`);
    return ipc_invoke("rpc_handler", "rpcReq", req).then(
      (r: RpcSuccess<Branch[]>) => r.result.data,
    );
  }

  async open(path: string): Promise<string> {
    const req = makeRpcRequest(`open_${this.cmd_suffix}`, { path });
    return ipc_invoke("rpc_handler", "rpcReq", req).then(
      (r: RpcSuccess<string>) => r.result.data,
    );
  }
}

export const repoFmc = new RepoFmc();
