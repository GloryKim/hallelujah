import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useRef, useState } from "react";
import { SIDECAR_CONFIG } from "../lib/sidecars";
import {
  appendTraffic,
  createEventId,
  type DataFlowState,
  type SidecarSnapshot,
  type TauriHealthRow,
  type TrafficEvent,
} from "../lib/traffic";

const POLL_MS = 5000;
const PULSE_MS = 900;

function initialState(): DataFlowState {
  return {
    sidecars: SIDECAR_CONFIG.map((s) => ({
      id: s.id,
      name: s.name,
      language: s.language,
      port: s.port,
      status: "unknown",
    })),
    traffic: [],
    tauriHealth: [],
    activePulse: null,
  };
}

export function useDataFlow() {
  const [state, setState] = useState<DataFlowState>(initialState);
  const probing = useRef(false);

  const flashPulse = useCallback(
    (channel: TrafficEvent["channel"], target?: string) => {
      setState((prev) => ({ ...prev, activePulse: { channel, target } }));
      window.setTimeout(() => {
        setState((prev) =>
          prev.activePulse?.channel === channel && prev.activePulse?.target === target
            ? { ...prev, activePulse: null }
            : prev
        );
      }, PULSE_MS);
    },
    []
  );

  const pushEvent = useCallback((event: TrafficEvent) => {
    setState((prev) => ({
      ...prev,
      traffic: appendTraffic(prev.traffic, event),
    }));
  }, []);

  const probeHttp = useCallback(
    async (sidecar: (typeof SIDECAR_CONFIG)[number]) => {
      const base = `http://127.0.0.1:${sidecar.port}`;
      const started = performance.now();

      pushEvent({
        id: createEventId(),
        timestamp: Date.now(),
        channel: "http",
        direction: "request",
        from: "React UI",
        to: sidecar.name,
        label: `GET ${base}/health`,
        ok: true,
      });
      flashPulse("http", sidecar.id);

      try {
        const health = await sidecar.client.health();
        const healthMs = Math.round(performance.now() - started);

        pushEvent({
          id: createEventId(),
          timestamp: Date.now(),
          channel: "http",
          direction: "response",
          from: sidecar.name,
          to: "React UI",
          label: `GET ${base}/health → 200`,
          payload: health,
          durationMs: healthMs,
          ok: health.ok,
        });

        const metaStarted = performance.now();
        pushEvent({
          id: createEventId(),
          timestamp: Date.now(),
          channel: "http",
          direction: "request",
          from: "React UI",
          to: sidecar.name,
          label: `GET ${base}/meta`,
          ok: true,
        });

        const meta = await sidecar.client.meta();
        const metaMs = Math.round(performance.now() - metaStarted);

        pushEvent({
          id: createEventId(),
          timestamp: Date.now(),
          channel: "http",
          direction: "response",
          from: sidecar.name,
          to: "React UI",
          label: `GET ${base}/meta → 200`,
          payload: meta,
          durationMs: metaMs,
          ok: true,
        });

        const snapshot: SidecarSnapshot = {
          id: sidecar.id,
          name: sidecar.name,
          language: sidecar.language,
          port: sidecar.port,
          status: health.ok ? "ok" : "error",
          lastHealth: health,
          lastMeta: meta,
          lastHttpLatencyMs: healthMs,
          lastCheckedAt: Date.now(),
        };

        setState((prev) => ({
          ...prev,
          sidecars: prev.sidecars.map((s) => (s.id === sidecar.id ? snapshot : s)),
        }));
      } catch (err) {
        const message = err instanceof Error ? err.message : "request failed";
        pushEvent({
          id: createEventId(),
          timestamp: Date.now(),
          channel: "http",
          direction: "error",
          from: sidecar.name,
          to: "React UI",
          label: `GET ${base} failed`,
          payload: message,
          durationMs: Math.round(performance.now() - started),
          ok: false,
        });

        setState((prev) => ({
          ...prev,
          sidecars: prev.sidecars.map((s) =>
            s.id === sidecar.id
              ? { ...s, status: "error", lastCheckedAt: Date.now() }
              : s
          ),
        }));
      }
    },
    [flashPulse, pushEvent]
  );

  const probeTauri = useCallback(async () => {
    const started = performance.now();

    pushEvent({
      id: createEventId(),
      timestamp: Date.now(),
      channel: "ipc",
      direction: "request",
      from: "React UI",
      to: "Tauri Shell",
      label: "invoke get_sidecar_status",
      ok: true,
    });
    flashPulse("ipc");

    try {
      const rows = await invoke<TauriHealthRow[]>("get_sidecar_status");
      const durationMs = Math.round(performance.now() - started);

      pushEvent({
        id: createEventId(),
        timestamp: Date.now(),
        channel: "ipc",
        direction: "response",
        from: "Tauri Shell",
        to: "React UI",
        label: "get_sidecar_status → OK",
        payload: rows,
        durationMs,
        ok: true,
      });

      for (const row of rows) {
        pushEvent({
          id: createEventId(),
          timestamp: Date.now(),
          channel: "http",
          direction: "request",
          from: "Tauri Shell",
          to: row.id,
          label: `GET http://127.0.0.1:${row.port}/health`,
          ok: true,
        });
        pushEvent({
          id: createEventId(),
          timestamp: Date.now(),
          channel: "http",
          direction: "response",
          from: row.id,
          to: "Tauri Shell",
          label: `GET /health → ${row.ok ? 200 : "error"}`,
          payload: { id: row.id, port: row.port, ok: row.ok },
          ok: row.ok,
        });
      }

      setState((prev) => ({
        ...prev,
        tauriHealth: rows,
        lastTauriProbeAt: Date.now(),
      }));
    } catch (err) {
      const message = err instanceof Error ? err.message : "invoke failed";
      pushEvent({
        id: createEventId(),
        timestamp: Date.now(),
        channel: "ipc",
        direction: "error",
        from: "Tauri Shell",
        to: "React UI",
        label: "get_sidecar_status failed",
        payload: message,
        durationMs: Math.round(performance.now() - started),
        ok: false,
      });
    }
  }, [flashPulse, pushEvent]);

  const probeAll = useCallback(async () => {
    if (probing.current) return;
    probing.current = true;
    try {
      await Promise.allSettled(SIDECAR_CONFIG.map((s) => probeHttp(s)));
      await probeTauri();
    } finally {
      probing.current = false;
    }
  }, [probeHttp, probeTauri]);

  useEffect(() => {
    probeAll();
    const id = window.setInterval(probeAll, POLL_MS);
    return () => window.clearInterval(id);
  }, [probeAll]);

  return { ...state, probeAll };
}
