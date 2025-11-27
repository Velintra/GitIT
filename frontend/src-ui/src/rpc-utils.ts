export type RpcId = number | string;

export type RpcRequest<P = any> = {
  jsonrpc: "2.0";
  id: RpcId;
  method: string;
  params: P;
};

export type RpcSuccess<T = any> = {
  jsonrpc: "2.0";
  id: RpcId;
  result: {
    data: T;
  };
};

export type RpcError = {
  jsonrpc: "2.0";
  id: RpcId;
  error: {
    code: number;
    message: string;
    data?: any;
  };
};

export type RpcResponse<T = any> = RpcSuccess<T> | RpcError;

export function makeRpcRequest<P = any>(
  method: string,
  params: P,
): RpcRequest<P> {
  return {
    jsonrpc: "2.0",
    id: 1,
    method,
    params,
  };
}
