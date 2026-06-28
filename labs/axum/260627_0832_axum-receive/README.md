# 260627_0832 — Axum JSON Receive + Log

Axum 서버가 JSON payload를 받아 `received.jsonl` 로그 파일에 한 줄씩 저장합니다.

## Run

**Terminal 1 — receive (먼저 실행)**

```bash
cd examples/axum/260627_0832_axum-receive
cargo run
```

**Terminal 2 — send**

```bash
cd examples/axum/260628_0831_axum-send
cargo run
```

Ctrl+C로 각각 종료합니다.

| Server | Default port | Env |
|--------|--------------|-----|
| receive | 8202 | `EXAMPLE_AXUM_RECEIVE_PORT`, `LOG_FILE` |
| send | (client) | `RECEIVER_URL=http://127.0.0.1:8202/ingest` |

## API

### `GET /health`

```json
{
  "ok": true,
  "service": "axum-receive",
  "total_received": 42,
  "log_file": "received.jsonl"
}
```

### `POST /ingest`

```json
{
  "stream": "fast-10ms",
  "sequence": 1,
  "interval_ms": 10,
  "value": 0.099833,
  "timestamp": 1719661440000
}
```

로그 파일 형식 (JSON Lines):

```json
{"received_at":1719661440001,"payload":{"stream":"fast-10ms","sequence":1,"interval_ms":10,"value":0.099833,"timestamp":1719661440000}}
```

## Verify

```bash
curl -s http://127.0.0.1:8202/health | jq
tail -f examples/axum/260627_0832_axum-receive/received.jsonl
```
