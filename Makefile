.PHONY: dev build build-sidecars check-ports generate clean \
        gin express fastapi nest

# ── full dev stack ────────────────────────────────────────────────────────────
dev:
	./scripts/dev.sh

# ── production build ──────────────────────────────────────────────────────────
build: build-sidecars
	pnpm --filter desktop tauri build

build-sidecars:
	./scripts/build-sidecars.sh

# ── individual sidecar builds ─────────────────────────────────────────────────
gin:
	$(MAKE) -C services/gin build

express:
	pnpm --filter express build:sidecar

fastapi:
	cd services/fastapi && bash scripts/build-sidecar.sh

nest:
	pnpm --filter nest-api build:sidecar

# ── utilities ─────────────────────────────────────────────────────────────────
check-ports:
	node scripts/check-ports.mjs

generate:
	node tools/sidecar-registry/generate.mjs

clean:
	rm -f apps/desktop/src-tauri/binaries/sc-*
	$(MAKE) -C services/gin clean
