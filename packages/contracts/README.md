# contracts

Shared contracts for all sidecars. This is the single source of truth for:

- Port assignments (`ports.yaml`)
- API specifications (`openapi/*.yaml`)

## Adding a new sidecar

1. Add an entry to `ports.yaml` (use the `7100` range)
2. Add `openapi/<service-id>.yaml`
3. Run `pnpm generate` from the root to regenerate TS/Rust constants

## Port ranges

| Range | Purpose |
|-------|---------|
| 5173 | Vite dev server |
| 7100–7199 | Sidecars |
