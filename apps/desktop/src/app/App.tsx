import { ArchitectureDiagram } from "../components/ArchitectureDiagram";
import { SidecarGrid } from "../components/SidecarGrid";
import { TrafficLog } from "../components/TrafficLog";
import { useDataFlow } from "../hooks/useDataFlow";
import "./App.css";

export function App() {
  const { sidecars, traffic, tauriHealth, lastTauriProbeAt, activePulse, probeAll } =
    useDataFlow();

  const online = sidecars.filter((s) => s.status === "ok").length;

  return (
    <div className="app">
      <header className="app-header">
        <div>
          <p className="eyebrow">Hallelujah Desktop</p>
          <h1>Data Flow Monitor</h1>
          <p className="subtitle">
            Visualizes HTTP calls from React and IPC probes through the Tauri shell
          </p>
        </div>
        <div className="header-actions">
          <div className="stat-pill">
            <span className="stat-value">
              {online}/{sidecars.length}
            </span>
            <span className="stat-label">sidecars online</span>
          </div>
          {lastTauriProbeAt != null && (
            <div className="stat-pill muted-pill">
              <span className="stat-value">
                {new Date(lastTauriProbeAt).toLocaleTimeString()}
              </span>
              <span className="stat-label">last Tauri probe</span>
            </div>
          )}
          <button type="button" className="probe-btn" onClick={() => probeAll()}>
            Probe now
          </button>
        </div>
      </header>

      <main className="app-main">
        <div className="main-left">
          <ArchitectureDiagram sidecars={sidecars} activePulse={activePulse} />
          <SidecarGrid sidecars={sidecars} tauriHealth={tauriHealth} />
        </div>
        <div className="main-right">
          <TrafficLog events={traffic} />
        </div>
      </main>
    </div>
  );
}
