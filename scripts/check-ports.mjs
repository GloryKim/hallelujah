#!/usr/bin/env node
/**
 * Reads ports.yaml and checks whether any registered ports are currently in use.
 * Exits with code 1 if a conflict is found.
 */

import { readFileSync } from "node:fs";
import { createConnection } from "node:net";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const yaml = readFileSync(
  resolve(__dirname, "../packages/contracts/ports.yaml"),
  "utf8"
);

// minimal YAML parser — only handles the simple scalar structure in ports.yaml
function parsePortsYaml(text) {
  const entries = [];
  let current = null;
  for (const line of text.split("\n")) {
    const serviceMatch = line.match(/^  (\w+):$/);
    if (serviceMatch) {
      current = { id: serviceMatch[1] };
      entries.push(current);
    }
    if (current) {
      const portMatch = line.match(/^\s+port:\s+(\d+)/);
      if (portMatch) current.port = Number(portMatch[1]);
    }
  }
  return entries.filter((e) => e.port);
}

function checkPort(port) {
  return new Promise((resolve) => {
    const conn = createConnection({ port, host: "127.0.0.1" });
    conn.on("connect", () => {
      conn.destroy();
      resolve(true); // port in use
    });
    conn.on("error", () => resolve(false));
    conn.setTimeout(500, () => {
      conn.destroy();
      resolve(false);
    });
  });
}

const services = parsePortsYaml(yaml);
let hasConflict = false;

await Promise.all(
  services.map(async ({ id, port }) => {
    const inUse = await checkPort(port);
    if (inUse) {
      console.error(`[check-ports] CONFLICT: ${id} port ${port} is already in use`);
      hasConflict = true;
    } else {
      console.log(`[check-ports] ok: ${id} port ${port} is free`);
    }
  })
);

if (hasConflict) process.exit(1);
