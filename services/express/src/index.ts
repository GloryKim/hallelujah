import { createServer } from "node:http";
import { createApp } from "./app.js";

const PORT = Number(process.env.SIDECAR_PORT ?? 7102);
const HOST = "127.0.0.1";

const app = createApp();
const server = createServer(app);

server.listen(PORT, HOST, () => {
  console.log(`[sc-express] listening on ${HOST}:${PORT}`);
});

// graceful shutdown on SIGTERM from Tauri
process.on("SIGTERM", () => {
  console.log("[sc-express] shutting down...");
  server.close(() => {
    console.log("[sc-express] stopped");
    process.exit(0);
  });
});
