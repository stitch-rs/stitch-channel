# Stitch Channel (`stitch-channel`)

[![CI](https://github.com/stitch-rs/stitch-channel/actions/workflows/ci.yml/badge.svg)](https://github.com/stitch-rs/stitch-channel/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust: 2024](https://img.shields.io/badge/Language-Rust%202024-orange.svg)](https://www.rust-lang.org/)

**Stitch Channel** (`stitch-channel`) is an ultra-low-latency, monomorphic communication channel and stream routing layer built for the **Sewing Machine Architecture (SMA)**.

Designed with **Mechanical Sympathy** for high-frequency trading (HFT), high-tick-rate game loops (`GoldSrc.rs`), and line-rate network telemetries (`ipatlas`), it replaces heavy, heap-allocating dynamic event buses with cache-line-isolated, zero-allocation ring buffers and static compile-time multicast.

---

## ⚡ Core Value Proposition

- **Zero Allocation on Hot Paths**: Bounded, pre-allocated SPSC and multicast channels with no heap allocations during steady-state dispatch.
- **Cache-Line Mechanical Isolation**: Atomic cursors reside on strict 64-byte aligned boundaries (`#[repr(align(64))]`), eliminating false sharing and cross-core cache invalidation storms (RFO).
- **Monomorphic U-Cycle Composition**: Plugs directly into `stitch-core` (`Layer` and `Terminal`), flattening dispatch into straight-line machine instructions without dynamic dispatch (`dyn`).
- **Static Monomorphic Fan-Out**: Tuple-unrolled compile-time multicast (`(A, B, C)`) bypassing heap arrays and virtual table lookups.
- **Pluggable Ecosystem Adapters**: First-class bridges for Tokio (`stitch-channel-adapter-tokio`) and NATS (`stitch-channel-adapter-nats`).

---

## 📦 Workspace Topology

| Component | Path | Description |
|---|---|---|
| `stitch-channel-core` | `crates/stitch-channel-core` | Pure `#![no_std]` ring buffers, cache-padded atomic cursors, and backpressure policies. |
| `stitch-channel` | `crates/stitch-channel` | High-level facade, fluent `ChannelBuilder`, SMA `Layer`/`Terminal` adapters, and static fan-out. |
| `stitch-channel-adapter-tokio` | `adapters/stitch-channel-adapter-tokio` | Bridge to `tokio::sync::mpsc` channels. |
| `stitch-channel-adapter-nats` | `adapters/stitch-channel-adapter-nats` | High-throughput streaming bridge to NATS message broker. |

---

## 🏃 Quickstart

```rust
use stitch_channel::prelude::*;

fn main() -> Result<(), ChannelError<u64>> {
    // 1. Initialize a 64-byte cacheline-isolated SPSC buffer with capacity 1024
    let buffer: SpscBuffer<u64, 1024> = ChannelBuilder::with_capacity::<1024>().build_spsc();
    let (mut tx, mut rx) = buffer.split();

    // 2. Lock-free zero-allocation send
    tx.try_send(42)?;

    // 3. Lock-free zero-allocation receive
    let val = rx.try_recv()?;
    assert_eq!(val, 42);

    Ok(())
}
```

---

## 📖 Deep Architectural Reference

For byte-level memory layouts, cacheline diagrams, hardware prefetcher sympathy, and memory ordering invariants, see [ARCHITECTURE.md](file:///D:/Repo/stitch-rs/stitch-channel/ARCHITECTURE.md).
