import { Router } from "express";

export const metaRouter = Router();

metaRouter.get("/", (_req, res) => {
  res.json({ service: "express", version: "0.1.0" });
});
