# Tauri App Learning Stack Research Report

> Date: 2026-06-28  
> Purpose: Document whether React, Express, Tokio, Axum, and similar technologies can be studied together in a Tauri app (frontend TS + backend RS), whether other web servers and examples can keep being added, and what problems that may cause

---

## 1. Executive Summary

| Area | Technology | Can you study it inside a Tauri app? | One-line assessment |
|------|------------|--------------------------------------|---------------------|
| Frontend | React + TypeScript | **Yes (recommended)** | Official Tauri templates include React + TS |
| Frontend | Express | **Not directly; workaround exists** | Frontend is a WebView (browser), not a Node.js/Express runtime |
| Backend | Rust syntax | **Yes (recommended)** | `src-tauri/` is a normal Rust project |
| Backend | Tokio | **Yes** | Tauri is already Tokio-based; use `tauri::async_runtime` |
| Backend | Axum | **Yes (pattern choice matters)** | Common pattern is an embedded localhost server in the same process |
| Other | Adding more web servers/examples | **Possible, but cost grows as you accumulate** | Port, runtime, and architecture confusion are the main risks |

**Key takeaway:** Tauri is optimized for “React + TS desktop UI + Rust native backend”. Express is not Tauri’s default backend; it is attached as a **separate Node process (sidecar)**. Axum and Tokio can be learned and used naturally inside the Rust backend.

---

## 2. Tauri App Structure (baseline understanding)

```
my-tauri-app/
├── src/                 # frontend (React + TypeScript + Vite, etc.)
├── package.json
└── src-tauri/           # Rust backend (Tauri core)
    ├── src/
    │   ├── main.rs
    │   └── lib.rs       # #[tauri::command], etc.
    ├── Cargo.toml
    └── tauri.conf.json
```

- **Frontend:** browser (WebView) environment. Uses web tech such as `fetch`, DOM, and React.
- **Backend:** Rust. Handles OS APIs, files, DB, networking, and other native work.
- **Default communication:** `invoke()` ↔ `#[tauri::command]` (IPC with JSON serialization).

Official docs: [Create a Project (Tauri v2)](https://v2.tauri.app/start/create-project/)  
React + TypeScript is officially supported in the `create-tauri-app` template.

---

## 3. Frontend (TypeScript) — React & Express

### 3.1 React + TypeScript → **study-friendly and a practical choice**

- Tauri v2 works with **almost any frontend framework**, including React, Vue, and Svelte.
- Typical learning/dev stack:
  - React + TypeScript + Vite
  - State management (Zustand, TanStack Query, etc.) is optional
- Calling Rust from the frontend:

```typescript
import { invoke } from "@tauri-apps/api/core";

const result = await invoke<string>("greet", { name: "world" });
```

- React components, hooks, TS types, routing, etc. can be studied **the same way as normal web React**.
- The main difference is that Node APIs (`fs`, `path`, `process`, etc.) are not available in the frontend. Delegate that work to Rust commands.

**Learning angle:** React + TS aligns 100% with the Tauri frontend learning path.

---

### 3.2 Express → **not inside the Tauri “frontend”; possible as a separate layer**

Express is a **Node.js HTTP server framework**. Unlike Electron, Tauri’s frontend does **not** embed a Node runtime.

| Aspect | Electron | Tauri |
|--------|----------|-------|
| Frontend JS environment | Chromium + Node combined | WebView (browser only) |
| Import Express in frontend code | Possible | **Not possible** |
| Run a Node server | Relatively easy | Requires sidecar or another design |

#### Ways to use Express with Tauri (for learning/experimentation)

1. **Separate learning project**  
   - Study Express in a standalone Node project such as `express-tutorial/`.  
   - HTTP, middleware, and routing concepts also help when learning Axum later.

2. **Sidecar pattern (official guide)**  
   - Package the Express app as a **single binary** with `pkg`, etc.  
   - Place it in `src-tauri/binaries/` and ship it with Tauri  
   - Spawn the process with `@tauri-apps/plugin-shell`  
   - Frontend calls `http://127.0.0.1:<port>`  
   - Reference: [Node.js as a sidecar (Tauri v2)](https://v2.tauri.app/learn/sidecar-nodejs/)

3. **Use Express only during development**  
   - Connect via Vite dev server proxy to an Express backend outside the Tauri app  
   - Fine for learning, but release builds need a sidecar or a Rust port

**Learning angle:**  
- “Study Express syntax inside Tauri frontend TS code” → **not possible**  
- “Build a REST API with Express and have the Tauri UI call it” → **possible (sidecar or external server)**  
- Express knowledge (routing, middleware, REST) is still a useful **conceptual precursor** for Axum/Rust HTTP learning

---

## 4. Backend (Rust) — syntax, Tokio, Axum

### 4.1 Studying Rust syntax → **yes (`src-tauri` is the learning space)**

- `src-tauri/` is a normal Cargo project.
- You can practice **Rust syntax in general** here: ownership, borrowing, enums, `Result`, traits, `async/await`, etc.
- Tauri-specific concepts:
  - `#[tauri::command]`
  - `tauri::Builder`, `generate_handler!`
  - capabilities / permissions (security settings)

Official docs: [Calling Rust from the Frontend](https://v2.tauri.app/develop/calling-rust/)

---

### 4.2 Tokio → **yes, but follow Tauri runtime rules**

- Tauri 2 uses a **Tokio async runtime** internally.
- For background async work, prefer **`tauri::async_runtime::spawn`** over `tokio::spawn`.
- You can write async commands as `#[tauri::command] async fn ...`.

```rust
#[tauri::command]
async fn heavy_task() -> Result<String, String> {
    // async Rust + Tokio patterns can be practiced here
    Ok("done".into())
}
```

**Learning angle:** Tokio syntax and patterns can be practiced in the Tauri backend. Just remember it runs inside the runtime Tauri manages.

---

### 4.3 Axum → **yes; embedded localhost server is the common pattern**

Representative patterns for using Axum with Tauri:

#### Pattern A: Tauri Commands only (simplest; Tauri’s recommended default)

- Frontend `invoke()` → direct Rust function call
- No HTTP server
- Good fit for single-user desktop UI

#### Pattern B: run Axum on localhost in the same process (common for learning and production)

```rust
// conceptual flow
// 1. in setup hook: tauri::async_runtime::spawn
// 2. TcpListener::bind("127.0.0.1:<port>")
// 3. axum::serve(listener, router).with_graceful_shutdown(...)
// 4. WebView loads http://127.0.0.1:<port> (or only calls the API)
```

- Real examples: [axum-tauri-example](https://github.com/itsuki-maru/axum-tauri-example), [crossplatform-rust-app](https://github.com/jazzenchen/crossplatform-rust-app)
- Advantage: you can study **web server framework patterns** such as REST, WebSocket, and middleware directly
- On shutdown, graceful shutdown with `CancellationToken` is recommended

#### Pattern C: tauri-plugin-axum (custom protocol)

- Connect the router via `axum://localhost/...`
- **Limitations:** not a real TCP port bind; WebSocket/streaming constraints ([wry#1404](https://github.com/tauri-apps/wry/issues/1404))
- If you need LAN serving or full HTTP features, **prefer Pattern B**

**Learning angle:** Axum + Tokio + the Rust HTTP ecosystem are **fully studyable** in the Tauri Rust backend.

---

## 5. Can you keep adding other web servers and examples?

### 5.1 Yes — but layers must stay distinct

In theory, a single Tauri app can contain all of the following **at once**:

| Layer | Example | Role |
|--------|---------|------|
| Tauri Commands | `invoke("save_file")` | UI ↔ Rust IPC |
| Axum (Rust) | `127.0.0.1:3000` | REST/WebSocket API |
| Actix-web, Warp, etc. (Rust) | another port | extra HTTP server (duplicate, not recommended) |
| Express sidecar (Node) | `127.0.0.1:4000` | reuse Node ecosystem |
| External example scripts | CLI, Python, etc. | sidecar / shell plugin |

For **learning and experimentation**, keeping multiple example folders and rotating them (“Axum example this week”, “command example next week”) is fine.

**Continuously accumulating frameworks in one app** causes the problems below.

---

### 5.2 Problems caused by accumulation

#### 1) Architecture confusion

- If the same feature exists in commands, an Axum API, and an Express API, it becomes unclear **which one is the source of truth**.
- Maintenance and debugging cost rises sharply.

**Recommendation:** limit an app to 1–2 communication styles  
- UI-focused: `invoke` + events  
- HTTP learning/external integration: **one** Axum localhost server

#### 2) Port collisions

- Overlapping ports such as Axum `3000`, Express sidecar `4000`, and Vite dev `5173` cause startup failures.
- Fix: centralize ports in env vars/config files; bind only to `127.0.0.1` for security.

#### 3) Tokio runtime conflicts

- Starting another full `#[tokio::main]` runtime outside Tauri’s control can cause conflicts or panics.
- **Inside the same process, use `tauri::async_runtime::spawn`.**

#### 4) Binary size and release complexity

- Express sidecar (`pkg` binary): adds several MB to tens of MB
- Multiple Rust HTTP stacks: duplicated dependencies and longer compile times
- Managing platform-specific sidecar binaries (`my-sidecar-x86_64-apple-darwin`, etc.)

#### 5) Security (CSP, capabilities)

- Tauri uses Content Security Policy and a capability-based permission model.
- Calls to `http://127.0.0.1`, shell sidecar spawn, and file access require **explicit permission**.
- Blindly copying examples often works in dev but fails in production builds.

#### 6) WebView vs localhost loading

- Depending on whether the WebView loads `https://tauri.localhost` (bundled assets) or `http://127.0.0.1:<port>` (served by Axum), behavior differs for:
  - HMR (hot reload)
  - WebSocket
  - mixed content
- If Axum also serves the frontend, document the **dev/prod URL strategy**.

#### 7) Broken types/builds when merging example code as-is

- Standalone Axum server examples often use `#[tokio::main]` in `main.rs` → conflicts with Tauri `main`/`lib` structure.
- Standalone React examples use Node APIs → runtime errors in the Tauri WebView.

**Recommendation:** keep examples in `examples/` or separate crates/workspaces, and **migrate only what you need** into the Tauri app.

---

## 6. Suggested Learning Roadmap (one Tauri app)

### Phase 1 — basic skeleton (1–2 weeks)

- `npm create tauri-app@latest` → choose React + TypeScript
- React components, hooks, TS types
- `#[tauri::command]` + `invoke()` round trip
- Rust basics (ownership, `Result`, enums)

### Phase 2 — Rust async (1–2 weeks)

- async commands
- `tauri::async_runtime::spawn`
- file I/O and simple background work
- events / Channel (progress streaming)

### Phase 3 — embedded Axum (2–3 weeks)

- add an Axum router module inside `src-tauri`
- 2–3 REST APIs on `127.0.0.1`
- call from React with `fetch("http://127.0.0.1:<port>/...")`
- graceful shutdown

### Phase 4 — Express (optional, parallel learning)

- **Express tutorial project separate from the Tauri app**
- or follow the sidecar guide for a minimal Express binary integration (including release and security)

This order lets you grow React/TS/Rust/Tokio/Axum naturally around one Tauri app, while studying Express **alongside** it for the Node ecosystem.

---

## 7. Decision Guide (quick reference)

| Goal | Recommended approach |
|------|----------------------|
| Build UI | React + TypeScript (Vite) |
| File/DB/OS access | `#[tauri::command]` |
| Practice Rust syntax | `src-tauri/src/*.rs` |
| async/await, Tokio | async command + `tauri::async_runtime` |
| REST API, middleware, WebSocket | Axum (localhost spawn) |
| Study Express middleware patterns | separate Node project or sidecar |
| Keep many examples | separate workspace / `examples/` folder |
| Three or more servers in one app | **not recommended** (clean up after learning) |

---

## 8. Final Answers (by question)

### Q1. Can I study React and Express on the frontend TS side?

- **React + TypeScript:** Yes. This is the standard Tauri frontend path and is supported by official templates.
- **Express:** Not inside frontend code, because there is no Node runtime. Express can be studied in (1) a separate Node project, or (2) integrated via a sidecar next to Tauri.

### Q2. Can I study Tokio, Axum, and Rust syntax on the backend RS side?

- **Yes.** `src-tauri` is the Rust learning space.
- Tokio is already used by Tauri, so practice async Rust with commands and spawn.
- Axum has a proven pattern as a localhost server in the same process.

### Q3. Can I keep adding other web server frameworks and examples to a Tauri app?

- **Yes.** But “trying examples for learning” and “permanently shipping everything in one binary” are different.
- Safer to keep examples separate and migrate only what you need.

### Q4. Does it cause problems?

- **No major issues if you separate concerns properly.**
- **Blind accumulation causes** port collisions, runtime conflicts, CSP/permission errors, bloated binaries, and architecture confusion.
- For real use and long-term learning, keep **Tauri commands + (optionally) one Axum server** as the default axis.

---

## 9. References

- [Tauri v2 — Create a Project](https://v2.tauri.app/start/create-project/)
- [Tauri v2 — Calling Rust (Commands)](https://v2.tauri.app/develop/calling-rust/)
- [Tauri v2 — Node.js sidecar](https://v2.tauri.app/learn/sidecar-nodejs/)
- [axum-tauri-example](https://github.com/itsuki-maru/axum-tauri-example)
- [crossplatform-rust-app (Tauri + Axum + React)](https://github.com/jazzenchen/crossplatform-rust-app)
- [tauri-plugin-axum (custom protocol, with limitations)](https://github.com/mcitem/tauri-plugin-axum)

---

*This document is a research report for planning Tauri learning in the hallelujah repository.*
