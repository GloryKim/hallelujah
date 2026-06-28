import type {
  DemoResponse,
  IngestRequest,
  IngestResponse,
  Message,
  RelayResponse,
} from "./types.js";

const SERVICE = "express";
const DEFAULT_PORT = 8202;
const DEFAULT_PEER = "http://127.0.0.1:8201";
const HEARTBEAT_MS = 5000;

export const port = Number(
  process.env.EXAMPLE_EXPRESS_PORT ?? process.env.PORT ?? DEFAULT_PORT,
);
export const peerUrl = (process.env.PEER_URL ?? DEFAULT_PEER).replace(/\/$/, "");

const messages: Message[] = [];
let heartbeatsSent = 0;

function nowMs(): number {
  return Date.now();
}

function newId(prefix: string): string {
  return `${prefix}-${nowMs()}`;
}

function reverseText(text: string): string {
  return [...text].reverse().join("");
}

export function getMessages(): Message[] {
  return [...messages];
}

export function getHeartbeatsSent(): number {
  return heartbeatsSent;
}

export function ingest(body: IngestRequest): IngestResponse {
  const stored: Message = {
    id: body.id ?? newId("ingest"),
    from: body.from,
    text: body.text,
    timestamp: body.timestamp ?? nowMs(),
  };

  messages.push(stored);

  const reply: Message = {
    id: newId("reply"),
    from: SERVICE,
    text: `ack from express: ${reverseText(body.text)}`,
    timestamp: nowMs(),
  };

  return { ok: true, stored, reply };
}

async function postJson<T>(url: string, body: unknown): Promise<T> {
  const res = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });

  if (!res.ok) {
    const text = await res.text();
    throw new Error(`HTTP ${res.status} ${url}: ${text}`);
  }

  return (await res.json()) as T;
}

export async function relay(text: string): Promise<RelayResponse> {
  const local: Message = {
    id: newId("relay"),
    from: SERVICE,
    text,
    timestamp: nowMs(),
  };
  messages.push(local);

  const peer = await postJson<IngestResponse>(`${peerUrl}/ingest`, {
    from: SERVICE,
    text,
    id: local.id,
    timestamp: local.timestamp,
  } satisfies IngestRequest);

  return { local, peer };
}

export async function demoRoundtrip(): Promise<DemoResponse> {
  const steps: DemoResponse["steps"] = [];

  steps.push({
    step: 1,
    actor: "express",
    action: "POST /relay to axum",
    payload: { text: "hello from express demo" },
  });

  const firstHop = await relay("hello from express demo");
  steps.push({
    step: 2,
    actor: "axum",
    action: "responded via /ingest",
    payload: firstHop.peer,
  });

  steps.push({
    step: 3,
    actor: "express",
    action: "ask axum to relay back",
    payload: { url: `${peerUrl}/relay`, text: "callback from express" },
  });

  const callback = await postJson<RelayResponse>(`${peerUrl}/relay`, {
    text: "callback from express",
  });
  steps.push({
    step: 4,
    actor: "axum",
    action: "relayed back to express /ingest",
    payload: callback,
  });

  return { steps };
}

export async function sendHeartbeat(): Promise<void> {
  try {
    await postJson<IngestResponse>(`${peerUrl}/ingest`, {
      from: SERVICE,
      text: "heartbeat",
      id: newId("hb"),
      timestamp: nowMs(),
    });
    heartbeatsSent += 1;
    console.log("[example-express] heartbeat ok -> axum");
  } catch (err) {
    console.error("[example-express] heartbeat error:", err);
  }
}

export function startHeartbeat(): NodeJS.Timeout {
  return setInterval(() => {
    void sendHeartbeat();
  }, HEARTBEAT_MS);
}
