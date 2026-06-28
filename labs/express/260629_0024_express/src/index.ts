import { createServer } from "node:http";
import { createApp, port } from "./app.js";
import { startHeartbeat } from "./peer.js";

const HOST = "127.0.0.1";
const app = createApp();
const server = createServer(app);

startHeartbeat();

server.listen(port, HOST, () => {
  console.log(`[example-express] listening on ${HOST}:${port}`);
});

process.on("SIGTERM", () => {
  console.log("[example-express] shutting down...");
  server.close(() => process.exit(0));
});

process.on("SIGINT", () => {
  console.log("[example-express] shutting down...");
  server.close(() => process.exit(0));
});
