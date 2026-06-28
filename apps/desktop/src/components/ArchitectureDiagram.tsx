import type { DataFlowState } from "../lib/traffic";

interface ArchitectureDiagramProps {
  sidecars: DataFlowState["sidecars"];
  activePulse: DataFlowState["activePulse"];
}

export function ArchitectureDiagram({ sidecars, activePulse }: ArchitectureDiagramProps) {
  const httpActive = activePulse?.channel === "http";
  const ipcActive = activePulse?.channel === "ipc";

  return (
    <section className="panel diagram">
      <header className="panel-header">
        <h2>Architecture</h2>
        <p>How data moves between the WebView, Tauri shell, and sidecars</p>
      </header>

      <div className="diagram-stack">
        <div className={`diagram-node ui ${ipcActive ? "pulse" : ""}`}>
          <span className="node-tag">Layer 1</span>
          <strong>React UI</strong>
          <span className="node-sub">WebView · fetch + invoke</span>
        </div>

        <div className={`diagram-link ipc ${ipcActive ? "active" : ""}`}>
          <span className="link-label">Tauri IPC</span>
          <span className="link-detail">invoke("get_sidecar_status")</span>
        </div>

        <div className={`diagram-node tauri ${ipcActive ? "pulse" : ""}`}>
          <span className="node-tag">Layer 2</span>
          <strong>Tauri Shell</strong>
          <span className="node-sub">Rust · spawn sidecars · reqwest probes</span>
        </div>

        <div className={`diagram-link http ${httpActive ? "active" : ""}`}>
          <span className="link-label">HTTP localhost</span>
          <span className="link-detail">GET /health · GET /meta</span>
        </div>

        <div className="diagram-sidecars">
          {sidecars.map((s) => {
            const lit = httpActive && activePulse?.target === s.id;
            return (
              <div
                key={s.id}
                className={`diagram-node sidecar status-${s.status} ${lit ? "pulse" : ""}`}
              >
                <strong>{s.id}</strong>
                <span className="node-sub">:{s.port}</span>
                <span className="node-lang">{s.language}</span>
              </div>
            );
          })}
        </div>
      </div>

      <div className="legend">
        <span className="legend-item">
          <i className="swatch ipc" /> IPC — React ↔ Tauri
        </span>
        <span className="legend-item">
          <i className="swatch http" /> HTTP — direct or via Rust
        </span>
      </div>
    </section>
  );
}
