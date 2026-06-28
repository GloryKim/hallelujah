import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Tauri dev server listens on a fixed port; do not change arbitrarily.
const TAURI_DEV_HOST = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    host: TAURI_DEV_HOST ?? "127.0.0.1",
    hmr: TAURI_DEV_HOST
      ? { protocol: "ws", host: TAURI_DEV_HOST, port: 5173 }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
