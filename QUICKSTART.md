# aiOS Rust - Quick Start

## Prerequisites

- Rust 1.70+ ([install from rustup.rs](https://rustup.rs/))
- Cargo (comes with Rust)

## Build

```bash
cd aiOS-rust
cargo build --release
```

## Run

```bash
cargo run --release
```

The server will start on `http://localhost:8765`

## Test

```bash
# Check status
curl http://localhost:8765/api/status

# Get capabilities
curl http://localhost:8765/api/capabilities
```

## Development

```bash
# Development build (faster compilation)
cargo build

# Run with debug logging
RUST_LOG=debug cargo run

# Run tests
cargo test
```

## Next Steps

This is a foundational implementation. The architecture is in place and ready for full implementation of:
- Mouse/keyboard control
- Window management  
- Screenshot with cursor reticle
- File operations
- Process management
- Vision analysis
- Task planning

See the main [README.md](README.md) for more details.

