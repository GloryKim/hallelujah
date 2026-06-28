.PHONY: build run tidy clean

BINARY := sc-gin
OUT_DIR := ../../apps/desktop/src-tauri/binaries

build:
	go build -o $(BINARY) ./cmd/server

# copy with the Tauri-required target triple suffix
build-sidecar:
	go build -o $(BINARY) ./cmd/server
	cp $(BINARY) $(OUT_DIR)/$(BINARY)-$$(rustc -vV | grep host | awk '{print $$2}')

run:
	SIDECAR_PORT=7101 go run ./cmd/server

tidy:
	go mod tidy

clean:
	rm -f $(BINARY)
