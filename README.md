# Ocrop

A Linux system metrics collector written in Rust. Reads process data 
from `/proc` and aggregates TCP connection stats.

## Architecture
- `agent` — reads /proc, computes CPU/memory metrics per process
- `aggregator` — collects TCP connection data
- `shared` — common types

## Run
cargo run --bin collector

## Notes
add count for TCP connections established
take into consideration cores when calculating CPU related metrics