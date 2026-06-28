import express from "express";
import { healthRouter } from "./routes/health.js";
import { metaRouter } from "./routes/meta.js";

export function createApp() {
  const app = express();
  app.use(express.json());

  app.use("/health", healthRouter);
  app.use("/meta", metaRouter);

  return app;
}
