import type { TrafficEvent } from "../lib/traffic";

interface TrafficLogProps {
  events: TrafficEvent[];
}

function formatTime(ts: number) {
  return new Date(ts).toLocaleTimeString("en-US", { hour12: false });
}

function directionArrow(direction: TrafficEvent["direction"]) {
  if (direction === "request") return "→";
  if (direction === "response") return "←";
  return "✕";
}

export function TrafficLog({ events }: TrafficLogProps) {
  return (
    <section className="panel traffic-log">
      <header className="panel-header">
        <h2>Traffic Log</h2>
        <p>Live request / response trail (newest first)</p>
      </header>

      <div className="traffic-list">
        {events.length === 0 && (
          <p className="muted empty">Waiting for the first probe…</p>
        )}
        {events.map((evt) => (
          <article
            key={evt.id}
            className={`traffic-row ${evt.ok ? "ok" : "err"} channel-${evt.channel}`}
          >
            <div className="traffic-row-top">
              <time>{formatTime(evt.timestamp)}</time>
              <span className={`chip channel-${evt.channel}`}>{evt.channel}</span>
              <span className="traffic-route">
                {evt.from} {directionArrow(evt.direction)} {evt.to}
              </span>
              {evt.durationMs != null && (
                <span className="traffic-ms">{evt.durationMs}ms</span>
              )}
            </div>
            <div className="traffic-label">{evt.label}</div>
            {evt.payload !== undefined && (
              <pre className="traffic-payload">
                {typeof evt.payload === "string"
                  ? evt.payload
                  : JSON.stringify(evt.payload, null, 2)}
              </pre>
            )}
          </article>
        ))}
      </div>
    </section>
  );
}
