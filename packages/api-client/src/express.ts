const BASE = "http://127.0.0.1:7102";

export interface HealthResponse {
  ok: boolean;
}

export interface MetaResponse {
  service: string;
  version: string;
}

async function get<T>(path: string): Promise<T> {
  const res = await fetch(`${BASE}${path}`);
  if (!res.ok) throw new Error(`sc-express ${path} → ${res.status}`);
  return res.json() as Promise<T>;
}

export const expressClient = {
  health: () => get<HealthResponse>("/health"),
  meta: () => get<MetaResponse>("/meta"),
};
