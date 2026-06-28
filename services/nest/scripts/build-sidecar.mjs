#!/usr/bin/env node
/**
 * Packages the NestJS API into a standalone binary using `pkg`,
 * then copies it with the Tauri target-triple suffix into binaries/.
 *
 * Usage: node scripts/build-sidecar.mjs
 */

import { execSync } from "node:child_process";
import { cpSync, mkdirSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = resolve(__dirname, "../../..");
const apiDir = resolve(__dirname, "../apps/api");
const binDir = resolve(root, "apps/desktop/src-tauri/binaries");

mkdirSync(binDir, { recursive: true });

// compile NestJS
execSync("pnpm build", { cwd: apiDir, stdio: "inherit" });

// bundle into a platform binary via pkg
execSync("pnpm pkg dist/main.js --output dist-bin/sc-nest", {
  cwd: apiDir,
  stdio: "inherit",
});

const triple = execSync("rustc -vV")
  .toString()
  .match(/host:\s+(\S+)/)?.[1];

if (!triple) throw new Error("could not detect Rust target triple");

const src = resolve(apiDir, "dist-bin/sc-nest");
const dest = resolve(binDir, `sc-nest-${triple}`);

cpSync(src, dest);
console.log(`[build-sidecar] copied sc-nest → ${dest}`);
