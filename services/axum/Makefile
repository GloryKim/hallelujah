.PHONY: build run clean build-sidecar

BINARY := sc-axum
OUT_DIR := ../../apps/desktop/src-tauri/binaries

build:
	cargo build --release

build-sidecar: build
	cp target/release/$(BINARY) $(OUT_DIR)/$(BINARY)-$$(rustc -vV | grep host | awk '{print $$2}')

run:
	SIDECAR_PORT=7105 cargo run

clean:
	cargo clean
