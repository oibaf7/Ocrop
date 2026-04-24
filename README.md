Brief structure description:
Project: Rust System Metrics Collector
What it is: A distributed system monitoring tool. Agents run on each server/container, collect process metrics, and push to a central aggregator. Aggregator stores and serves data to a frontend dashboard. Eventual goal: deploy on a friend's multi-server setup, with a Raspberry Pi Pico W as a physical display node.
Architecture:
agent → aggregator : raw TCP
aggregator → frontend : HTTP (axum)

Stack decisions:
Agent: std only + serde_json. No sysinfo — read /proc directly
Aggregator: std + serde_json + axum
Transport: raw TCP with length-prefixed JSON payload
Local testing: Docker containers simulating multiple servers
Repo: single monorepo, Cargo workspace with agent/, aggregator/, shared/ crates
What to read from /proc:
/proc/{pid}/status — memory
/proc/{pid}/stat — CPU time
/proc directory — process enumeration
v1 scope (keep tight):
Agent collects per-process CPU and memory, pushes to aggregator every N seconds
Aggregator receives, stores in-memory, exposes via HTTP
Docker setup with multiple agent instances
Basic frontend showing current stats
Relevant terms: process monitoring, metrics collection, observability, telemetry, pull vs push metrics, Prometheus data model (worth understanding even if not using)
Timeline: June, before the database engine in July-August. Estimated 2-4 weeks depending on scope discipline.
Monorepo structure:
metrics/
  agent/
  aggregator/
  shared/

