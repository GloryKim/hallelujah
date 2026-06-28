export type TrafficChannel = "http" | "ipc";
export type TrafficDirection = "request" | "response" | "error";

export interface TrafficEvent {
  id: string;
  timestamp: number;
  channel: TrafficChannel;
  direction: TrafficDirection;
  from: string;
  to: string;
  label: string;
  payload?: unknown;
  durationMs?: number;
  ok: boolean;
}

export interface SidecarSnapshot {
  id: string;
  name: string;
  language: string;
  port: number;
  status: "unknown" | "ok" | "error";
  lastHealth?: { ok: boolean };
  lastMeta?: { service: string; version: string };
  lastHttpLatencyMs?: number;
  lastCheckedAt?: number;
}

export interface TauriHealthRow {
  id: string;
  port: number;
  ok: boolean;
}

export interface DataFlowState {
  sidecars: SidecarSnapshot[];
  traffic: TrafficEvent[];
  tauriHealth: TauriHealthRow[];
  lastTauriProbeAt?: number;
  activePulse: { channel: TrafficChannel; target?: string } | null;
}

let eventCounter = 0;

export function createEventId(): string {
  eventCounter += 1;
  return `evt-${Date.now()}-${eventCounter}`;
}

export function appendTraffic(
  events: TrafficEvent[],
  event: TrafficEvent,
  max = 80
): TrafficEvent[] {
  return [event, ...events].slice(0, max);
}
