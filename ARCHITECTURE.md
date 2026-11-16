# Architectural & Mechanical Specification: `stitch-channel`

**Ecosystem:** Stitch Sewing Machine Architecture (SMA) Monomorphic Pipeline  
**Version:** 0.1.0  
**Classification:** Core Subsystem Specification  

---

## 1. System Taxonomy & Separation of Concerns

`stitch-channel` sits at the pipeline interconnection boundary of the Stitch ecosystem:

```text
┌─────────────────────────────────────────────────────────────┐
│                     EXTERNAL RUNTIMES                       │
│      Tokio Task Pool        │          NATS Cluster         │
└─────────────────────────────┬───────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                 ADAPTER LAYER (Policy vs Mechanism)         │
│  stitch-channel-adapter-tokio  │ stitch-channel-adapter-nats│
└─────────────────────────────┬───────────────────────────────┘
                              ▼ (Zero-Copy Transfer)
┌─────────────────────────────────────────────────────────────┐
│                SMA PIPELINE INTERCONNECT                    │
│    stitch-channel (Builders, Fanout, Layer & Terminal)      │
│  crates/stitch-channel-core (SPSC, Padded Cursors, Policy)  │
└─────────────────────────────┬───────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     HARDWARE / CPU L1D                      │
│     64-Byte Cache Lines     │    Branch-Predictor Inlining  │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. User Stories (DX) & Machine Stories (MX)

### Developer Experience (DX)
- **US-01 (Type-Safe Zero-Copy Transfer):** As a systems developer, I want typed `Producer<T>` and `Consumer<T>` endpoints that operate directly on stack or pre-allocated rings, ensuring zero heap allocations on hot paths.
- **US-02 (Monomorphic SMA Composition):** As an SMA pipeline architect, I want `ChannelEgressTerminal` and `ChannelIngressLayer` implementing `Terminal` and `Layer`, so channel dispatches are flattened by LLVM without virtual calls (`dyn`).
- **US-03 (Explicit Backpressure Contracts):** As a reliability engineer, I want explicit backpressure variants (`Reject`, `DropOldest`, `Block`) returning typed errors (`Full(T)`, `Disconnected`, `Lagged`), eliminating silent message drops.
- **US-04 (Zero-Cost Static Multicast):** As a game server developer, I want to fan out state updates across subscribers using compile-time tuples `(A, B, C)` with zero heap arrays.

### Machine Experience (MX / Mechanical Sympathy)
- **MS-01 (L1D Cache Line Isolation & False Sharing Elimination):** As an L1 Data Cache line (64 bytes), I want `head` and `tail` cursors to reside on separate 64-byte boundaries with `#[repr(align(64))]`, preventing cross-core Request-For-Ownership (RFO) invalidation storms.
- **MS-02 (Hardware Stream Prefetcher Friendliness):** As the CPU L1/L2 Stream Prefetcher, I want ring backing memory to be contiguous and power-of-two sized, so advancing indices translates to monotonic linear memory strides that trigger hardware stream prefetchers.
- **MS-03 (Branch Predictor & Straight-Line Monomorphization):** As the CPU Branch Predictor and Instruction Cache (L1I), I want send and receive operations to be marked `#[inline(always)]`, compiling into branch-predicted straight-line assembly (`mov`, `and`, `cmp`, `cmovnz`) without indirect call penalties (`call rax`).
- **MS-04 (Store Buffer Drain Minimization):** As the CPU Memory Subsystem, I want local cached shadow cursors and explicit `Acquire`/`Release` memory orderings (avoiding blanket `SeqCst`), reducing atomic bus locks and store-buffer flushes.

---

## 3. Physical Memory Layout & Concurrency Contracts

```
        64-Byte Cache Line 0                64-Byte Cache Line 1
+-----------------------------------+ +-----------------------------------+
| Producer Cursors & Local Cached   | | Consumer Cursors & Local Cached   |
| - head: AtomicUsize (Release)     | | - tail: AtomicUsize (Release)     |
| - cached_tail: usize (Local)      | | - cached_head: usize (Local)      |
| - 48 bytes reserved padding       | | - 48 bytes reserved padding       |
+-----------------------------------+ +-----------------------------------+
                  |                                     |
                  +==================+==================+
                                     |
                      64-Byte Aligned Contiguous Ring
+-------------------------------------------------------------------------+
| [ Slot 0 ] [ Slot 1 ] [ Slot 2 ] ... [ Slot N-1 ] (Power of 2 Capacity) |
+-------------------------------------------------------------------------+
```

### Shadow Cached Cursors
1. **Producer Side**: The producer maintains a local non-atomic `cached_tail`. It only issues an atomic `tail.load(Ordering::Acquire)` when `head.wrapping_sub(cached_tail) >= CAP`. This eliminates up to 98% of cross-core atomic bus reads in saturated streaming loops.
2. **Consumer Side**: Symmetrically, the consumer maintains a local non-atomic `cached_head` and only issues `head.load(Ordering::Acquire)` when `tail == cached_head`.
3. **Power-of-Two Indexing**: Fast bitwise modulo `index = cursor & (CAP - 1)` replaces integer division (`div`/`rem`), saving 12–40 CPU cycles per item.
