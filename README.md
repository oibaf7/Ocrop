# Rust System Metrics Collector

## What It Is
A distributed system monitoring tool. Agents run on each server/container, collect process metrics, and push to a central aggregator. Aggregator stores and serves data to a frontend dashboard. Eventual goal: deploy on a friend's multi-server setup, with a Raspberry Pi Pico W as a physical display node.

---

## Architecture

```
agent  →  aggregator  :  raw TCP
aggregator  →  frontend  :  HTTP (axum)
```

---

## Stack Decisions

| Layer | Choice |
|---|---|
| Agent | `std` only + `serde_json` — no `sysinfo`, read `/proc` directly |
| Aggregator | `std` + `serde_json` + `axum` |
| Transport | Raw TCP with length-prefixed JSON payload |
| Local testing | Docker containers simulating multiple servers |
| Repo | Single monorepo, Cargo workspace |

---

## Monorepo Structure

```
metrics/
├── agent/
├── aggregator/
└── shared/
```

---

## What to Read from `/proc`

| Path | Data |
|---|---|
| `/proc/{pid}/status` | Memory |
| `/proc/{pid}/stat` | CPU time |
| `/proc/` | Process enumeration |

---

## v1 Scope

- Agent collects per-process CPU and memory, pushes to aggregator every N seconds
- Aggregator receives, stores in-memory, exposes via HTTP
- Docker setup with multiple agent instances
- Basic frontend showing current stats

---

## Relevant Terms

`process monitoring` · `metrics collection` · `observability` · `telemetry` · `pull vs push metrics` · `Prometheus data model`