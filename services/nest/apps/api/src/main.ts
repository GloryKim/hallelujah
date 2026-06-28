import "reflect-metadata";
import { NestFactory } from "@nestjs/core";
import { AppModule } from "./app.module";

async function bootstrap() {
  const port = Number(process.env.SIDECAR_PORT ?? 7104);
  const app = await NestFactory.create(AppModule);

  // Reflect the request origin so the Tauri WebView dev server can fetch this API.
  app.enableCors({
    origin: true,
    methods: ["GET", "HEAD", "OPTIONS"],
    allowedHeaders: ["Content-Type"],
  });

  await app.listen(port, "127.0.0.1");
  console.log(`[sc-nest] listening on 127.0.0.1:${port}`);

  // graceful shutdown on SIGTERM from Tauri
  process.on("SIGTERM", async () => {
    console.log("[sc-nest] shutting down...");
    await app.close();
    console.log("[sc-nest] stopped");
    process.exit(0);
  });
}

bootstrap();
