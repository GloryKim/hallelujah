#!/usr/bin/env node
/**
 * Packages the Express server into a standalone binary using `pkg`,
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
const binDir = resolve(root, "apps/desktop/src-tauri/binaries");

mkdirSync(binDir, { recursive: true });

// compile TypeScript first
execSync("pnpm build", { cwd: resolve(__dirname, ".."), stdio: "inherit" });

// bundle into a platform binary via pkg
execSync("pnpm pkg dist/index.js --output dist-bin/sc-express", {
  cwd: resolve(__dirname, ".."),
  stdio: "inherit",
});

// resolve the Tauri target triple from rustc
const triple = execSync("rustc -vV")
  .toString()
  .match(/host:\s+(\S+)/)?.[1];

if (!triple) throw new Error("could not detect Rust target triple");

const src = resolve(__dirname, "../dist-bin/sc-express");
const dest = resolve(binDir, `sc-express-${triple}`);

cpSync(src, dest);
console.log(`[build-sidecar] copied sc-express → ${dest}`);
