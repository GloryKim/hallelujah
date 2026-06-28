import express from "express";
import {
  demoRoundtrip,
  getHeartbeatsSent,
  getMessages,
  ingest,
  peerUrl,
  port,
  relay,
} from "./peer.js";

export function createApp() {
  const app = express();
  app.use(express.json());

  app.get("/health", (_req, res) => {
    res.json({
      ok: true,
      service: "express",
      peer_url: peerUrl,
      message_count: getMessages().length,
      heartbeats_sent: getHeartbeatsSent(),
    });
  });

  app.get("/messages", (_req, res) => {
    res.json(getMessages());
  });

  app.post("/ingest", (req, res) => {
    res.json(ingest(req.body));
  });

  app.post("/relay", async (req, res) => {
    const text = String(req.body?.text ?? "");
    if (!text) {
      res.status(400).json({ error: "text is required" });
      return;
    }

    try {
      res.json(await relay(text));
    } catch (err) {
      res.status(502).json({
        error: err instanceof Error ? err.message : "peer request failed",
      });
    }
  });

  app.get("/demo/roundtrip", async (_req, res) => {
    try {
      res.json(await demoRoundtrip());
    } catch (err) {
      res.status(502).json({
        error: err instanceof Error ? err.message : "demo failed",
      });
    }
  });

  return app;
}

export { port };
