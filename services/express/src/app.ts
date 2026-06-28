import express from "express";
import { healthRouter } from "./routes/health.js";
import { metaRouter } from "./routes/meta.js";

export function createApp() {
  const app = express();

  // Allow fetch from the Tauri WebView dev server (localhost:5173).
  app.use((_req, res, next) => {
    res.setHeader("Access-Control-Allow-Origin", "*");
    res.setHeader("Access-Control-Allow-Methods", "GET, OPTIONS");
    res.setHeader("Access-Control-Allow-Headers", "Content-Type");
    next();
  });
  app.options("*", (_req, res) => res.sendStatus(204));

  app.use(express.json());

  app.use("/health", healthRouter);
  app.use("/meta", metaRouter);

  return app;
}
