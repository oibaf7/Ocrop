# Ocrop

A distributed Linux process monitor written in Rust. Agents collect per-process
CPU and memory metrics from `/proc` and stream them to a central aggregator 
application layer protocol over UDP using Pulse, a lightweight custom transport protocol, with a live
Ratatui TUI.

## Architecture

- `agent` — reads `/proc`, computes per-process CPU (delta-based) and PSS memory, sends snapshots over Pulse
- `aggregator` — receives from multiple agents, tracks per-agent connection state, renders a live TUI with per-machine tab switching
- `shared` — common types and serialization
- `pulse` — custom UDP application layer protocol: fire-and-forget sends on a fixed interval, with expiry-based retransmission requests when an agent goes quiet

### Pulse

Pulse is a simple, ack-less UDP protocol built for periodic metric snapshots:

- Agents send a snapshot every few seconds; no acknowledgements are expected
- The aggregator tracks the last snapshot id seen per agent
- If no new data arrives within a timeout, the aggregator sends a retransmission
  request carrying the last id it received — a cumulative "catch me up" signal,
  not a request for a specific lost packet
- The sender replies with its latest snapshot if it's still fresh enough to be
  useful, otherwise the request is ignored
- Snapshots are serialized with bincode to stay well under typical MTU limits

## Run

```bash
# start aggregator
cargo run --bin aggregator

# start agent (on same or remote machine)
cargo run --bin agent
```

## Config

Each binary has a `Config.toml` for address, timeout, and polling interval.

## TUI
<img width="2714" height="1246" alt="image" src="https://github.com/user-attachments/assets/ade2d4f9-4ed1-46fd-a4ea-6a37bff97919" />