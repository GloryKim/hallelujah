import { Controller, Get } from "@nestjs/common";

@Controller()
export class HealthController {
  @Get("health")
  health() {
    return { ok: true };
  }

  @Get("meta")
  meta() {
    return { service: "nest", version: "0.1.0" };
  }
}
