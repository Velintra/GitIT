import { ipc_invoke } from "../ipc";

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

  async open(path: string): Promise<string> {
    return ipc_invoke("rpc_handler", "rpcReq", {
      jsonrpc: "2.0",
      id: 1,
      method: `open_${this.cmd_suffix}`,
      params: { path },
    }).then((r) => r.result.data);
  }
}

export const repoFmc = new RepoFmc();
