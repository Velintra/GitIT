import { Stronghold, Location, Client } from "@tauri-apps/plugin-stronghold";
import { appDataDir } from "@tauri-apps/api/path";

export async function initStronghold(name: string, pwd: string) {
  const vaultPath = `${await appDataDir()}/vault.hold`;
  const stronghold = await Stronghold.load(vaultPath, pwd);

  let client: Client;
  try {
    client = await stronghold.loadClient(name);
  } catch {
    client = await stronghold.createClient(name);
  }
  return {
    stronghold,
    client,
  };
}

async function insertToVault(store: any, key: string, value: string) {
  const data = Array.from(new TextEncoder().encode(value));
  await store.insert(key, data);
}

async function getVaultRecord(store: any, key: string): Promise<string> {
  const data = await store.get(key);
  return new TextDecoder().decode(new Uint8Array(data));
}
