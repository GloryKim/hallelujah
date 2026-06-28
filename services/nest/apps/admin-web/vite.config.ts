import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Admin UI is served by the Nest sidecar on a separate port, not the Tauri window.
export default defineConfig({
  plugins: [react()],
  server: {
    port: 7105,
    host: "127.0.0.1",
  },
  build: {
    outDir: "../../api/public",
  },
});
