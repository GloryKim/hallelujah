import { useEffect, useState } from "react";
import { ginClient, expressClient, fastapiClient, nestClient } from "@hallelujah/api-client";

type ServiceStatus = "unknown" | "ok" | "error";

interface SidecarState {
  gin: ServiceStatus;
  express: ServiceStatus;
  fastapi: ServiceStatus;
  nest: ServiceStatus;
}

export function App() {
  const [status, setStatus] = useState<SidecarState>({
    gin: "unknown",
    express: "unknown",
    fastapi: "unknown",
    nest: "unknown",
  });

  useEffect(() => {
    async function pollAll() {
      const probe = async (
        key: keyof SidecarState,
        fn: () => Promise<{ ok: boolean }>
      ) => {
        try {
          const res = await fn();
          setStatus((prev) => ({ ...prev, [key]: res.ok ? "ok" : "error" }));
        } catch {
          setStatus((prev) => ({ ...prev, [key]: "error" }));
        }
      };

      await Promise.allSettled([
        probe("gin", ginClient.health),
        probe("express", expressClient.health),
        probe("fastapi", fastapiClient.health),
        probe("nest", nestClient.health),
      ]);
    }

    pollAll();
    const id = setInterval(pollAll, 5000);
    return () => clearInterval(id);
  }, []);

  const dot = (s: ServiceStatus) =>
    s === "ok" ? "🟢" : s === "error" ? "🔴" : "⚪";

  return (
    <div style={{ fontFamily: "monospace", padding: "2rem" }}>
      <h1>Hallelujah</h1>
      <h2>Sidecar Status</h2>
      <ul>
        <li>{dot(status.gin)} gin (7101)</li>
        <li>{dot(status.express)} express (7102)</li>
        <li>{dot(status.fastapi)} fastapi (7103)</li>
        <li>{dot(status.nest)} nest (7104)</li>
      </ul>
    </div>
  );
}
