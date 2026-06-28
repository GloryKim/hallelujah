import type { SidecarSnapshot } from "../lib/traffic";

interface SidecarGridProps {
  sidecars: SidecarSnapshot[];
  tauriHealth: { id: string; port: number; ok: boolean }[];
}

function statusLabel(status: SidecarSnapshot["status"]) {
  if (status === "ok") return "Online";
  if (status === "error") return "Offline";
  return "Unknown";
}

export function SidecarGrid({ sidecars, tauriHealth }: SidecarGridProps) {
  const tauriById = new Map(tauriHealth.map((r) => [r.id, r]));

  return (
    <section className="panel">
      <header className="panel-header">
        <h2>Sidecars</h2>
        <p>Latest payloads received over HTTP from the React layer</p>
      </header>

      <div className="sidecar-grid">
        {sidecars.map((s) => {
          const tauriRow = tauriById.get(s.name);
          return (
            <article key={s.id} className={`sidecar-card status-${s.status}`}>
              <div className="sidecar-card-head">
                <div>
                  <h3>{s.id}</h3>
                  <span className="sidecar-meta">
                    {s.language} · {s.name} · :{s.port}
                  </span>
                </div>
                <span className={`badge status-${s.status}`}>{statusLabel(s.status)}</span>
              </div>

              <dl className="payload-list">
                <div>
                  <dt>/health</dt>
                  <dd>
                    {s.lastHealth ? (
                      <code>{JSON.stringify(s.lastHealth)}</code>
                    ) : (
                      <span className="muted">—</span>
                    )}
                  </dd>
                </div>
                <div>
                  <dt>/meta</dt>
                  <dd>
                    {s.lastMeta ? (
                      <code>{JSON.stringify(s.lastMeta)}</code>
                    ) : (
                      <span className="muted">—</span>
                    )}
                  </dd>
                </div>
                <div>
                  <dt>Tauri probe</dt>
                  <dd>
                    {tauriRow ? (
                      <code>{JSON.stringify(tauriRow)}</code>
                    ) : (
                      <span className="muted">—</span>
                    )}
                  </dd>
                </div>
              </dl>

              <footer className="sidecar-card-foot">
                {s.lastHttpLatencyMs != null && (
                  <span>{s.lastHttpLatencyMs}ms HTTP</span>
                )}
                {s.lastCheckedAt != null && (
                  <span>{new Date(s.lastCheckedAt).toLocaleTimeString()}</span>
                )}
              </footer>
            </article>
          );
        })}
      </div>
    </section>
  );
}
