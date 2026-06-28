# contracts

Shared contracts for all sidecars. This is the single source of truth for:

- Port assignments (`ports.yaml`)
- API specifications (`openapi/*.yaml`)

## Adding a new sidecar

1. Add an entry to `ports.yaml` (use the `7100` range)
2. Add `openapi/<service-id>.yaml`
3. Run `pnpm generate` from the root to regenerate TS/Rust constants
4. Add the service to dev/build scripts and Tauri sidecar configuration
5. Add CI coverage for at least build or syntax validation

## Contract expectations

Every OpenAPI file should include:

- `GET /health` returning `{ "ok": true }`
- `GET /meta` returning `{ "service": "<name>", "version": "0.1.0" }`
- A localhost `servers` entry matching the registered port in `ports.yaml`

Keep API specs, handwritten clients, and sidecar implementations in the same pull request when endpoint shapes change.

## Port ranges

| Range | Purpose |
|-------|---------|
| 5173 | Vite dev server |
| 7100–7199 | Sidecars |
