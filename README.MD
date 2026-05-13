# PipeWire Telemetry RS

A high-performance, real-time safe Digital Signal Processing (DSP) engine and asynchronous telemetry bridge for PipeWire, written in pure Rust.

## The Problem
Modern Linux audio (PipeWire) operates in a high-priority, strictly timed environment. Traditional asynchronous Rust (Tokio) and UI frameworks (Ratatui, GTK) are too slow and non-deterministic to live inside the audio callback. Using standard locks (`Mutex`) or memory allocation (`Vec`) in the audio thread causes audible glitches and system instability.

## The Solution
`pipewire-telemetry-rs` acts as a zero-copy, lock-free bridge. It captures raw PCM data from PipeWire and moves it into an atomic ring buffer. This allows high-level applications to consume complex audio telemetry (like RMS levels and FFT frequency bins) without ever interrupting the audio stream.

## Core Architecture

- **FFI Quarantine (`ffi.rs`):** Strictly isolated unsafe blocks managing the PipeWire C-headers.
- **Lock-free Bridge (`buffer.rs`):** An atomic SPSC (Single-Producer Single-Consumer) ring buffer for telemetry data.
- **Pure DSP (`dsp/`):** A collection of mathematical modules (RMS, FFT, Peak detection) that operate on stack-allocated arrays.
- **Async Consumer:** A clean, Tokio-compatible API for frontend developers.

## Roadmap

### Phase 1: The Foundation
- [ ] Connect to PipeWire via `pipewire-rs` FFI.
- [ ] Implement a non-blocking monitoring node.
- [ ] Establish the Lock-free Ring Buffer.

### Phase 2: Time-Domain Telemetry
- [ ] Implement Root Mean Square (RMS) calculation for volume tracking.
- [ ] Peak amplitude detection.
- [ ] Silence/Signal threshold triggers.

### Phase 3: Frequency-Domain Telemetry
- [ ] Fast Fourier Transform (FFT) integration.
- [ ] Frequency binning for visualizers (Bass, Mids, Treble).
- [ ] Beat and transient detection heuristics.

## Example Usage (Projected API)

```rust
use pipewire_telemetry::{Engine, AudioEvent};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut engine = Engine::new().await?;
    let mut rx = engine.subscribe();

    println!("Monitoring audio stream...");

    while let Some(event) = rx.recv().await {
        match event {
            AudioEvent::Rms(level) => println!("Volume: {:.2} dB", level),
            AudioEvent::Beat => println!("🥁 BEAT DETECTED"),
            _ => {}
        }
    }
    Ok(())
}