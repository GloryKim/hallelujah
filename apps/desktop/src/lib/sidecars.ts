import {
  axumClient,
  expressClient,
  fastapiClient,
  ginClient,
  nestClient,
  SIDECAR_PORTS,
  type SidecarId,
} from "@hallelujah/api-client";
import type { SidecarSnapshot } from "./traffic";

export interface SidecarConfig {
  id: SidecarId;
  name: string;
  language: string;
  port: number;
  client: {
    health: () => Promise<{ ok: boolean }>;
    meta: () => Promise<{ service: string; version: string }>;
  };
}

export const SIDECAR_CONFIG: SidecarConfig[] = [
  { id: "gin", name: "sc-gin", language: "Go", port: SIDECAR_PORTS.gin, client: ginClient },
  {
    id: "express",
    name: "sc-express",
    language: "Node",
    port: SIDECAR_PORTS.express,
    client: expressClient,
  },
  {
    id: "fastapi",
    name: "sc-fastapi",
    language: "Python",
    port: SIDECAR_PORTS.fastapi,
    client: fastapiClient,
  },
  { id: "nest", name: "sc-nest", language: "NestJS", port: SIDECAR_PORTS.nest, client: nestClient },
  { id: "axum", name: "sc-axum", language: "Rust", port: SIDECAR_PORTS.axum, client: axumClient },
];

export function initialSidecarSnapshots(): SidecarSnapshot[] {
  return SIDECAR_CONFIG.map((s) => ({
    id: s.id,
    name: s.name,
    language: s.language,
    port: s.port,
    status: "unknown",
  }));
}
