# 260628_0831 — Axum JSON Send (10ms / 30ms)

receive 서버(`/ingest`)로 JSON을 **10ms**, **30ms** 간격으로 보내는 클라이언트 예제입니다.

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

| Variable | Default | Description |
|----------|---------|-------------|
| `RECEIVER_URL` | `http://127.0.0.1:8202/ingest` | receive 서버 URL |

## Payload

두 개의 스트림이 동시에 전송됩니다.

| Stream | Interval | JSON `stream` field |
|--------|----------|---------------------|
| fast | 10ms | `fast-10ms` |
| slow | 30ms | `slow-30ms` |

```json
{
  "stream": "fast-10ms",
  "sequence": 1,
  "interval_ms": 10,
  "value": 0.099833,
  "timestamp": 1719661440000
}
```

receive 쪽 `received.jsonl`에서 결과를 확인하세요.
