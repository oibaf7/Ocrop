# Ocrop

A distributed Linux process monitor written in Rust. Agents collect per-process 
CPU and memory metrics from `/proc` and stream them over TCP to a central aggregator 
with a live Ratatui TUI.

## Architecture

- `agent` — reads `/proc`, computes per-process CPU (delta-based) and PSS memory, streams over TCP
- `aggregator` — receives from multiple agents, renders a live TUI with per-machine tab switching
- `shared` — common types and serialization

## Run

```bash
# start aggregator
cargo run --bin aggregator

# start agent (on same or remote machine)
cargo run --bin agent
```

## Config

Each binary has a `Config.toml` for timeout and polling interval.

## TUI
<img width="2714" height="1246" alt="image" src="https://github.com/user-attachments/assets/ade2d4f9-4ed1-46fd-a4ea-6a37bff97919" />

## Notes
- add count for TCP connections established
- implement into app processes aggregator rendering
- think about concurrency for hash map
