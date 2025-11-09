# aiOS (Rust) - AI Native Operating System Interface

A high-performance, system-level AI operating system interface built in Rust. Provides comprehensive computer control capabilities for AI agents with native system access and optimal performance.

## Why Rust?

- **Performance**: Zero-cost abstractions, no garbage collection overhead
- **Safety**: Memory safety without runtime overhead
- **System Access**: Direct access to OS APIs and kernel-level operations
- **Concurrency**: Excellent async/await support with Tokio for async operations
- **Safety**: Compile-time guarantees prevent entire classes of bugs

## Features

- **High-Performance Action Execution**: Native system calls for maximum speed
- **System-Level Operations**: Direct OS API access (Windows, Linux, macOS)
- **Kernel-Level Capabilities**: Foundation for kernel-level operations
- **Memory Safety**: Rust's ownership system prevents memory bugs
- **Concurrent Operations**: Safe concurrent task execution
- **REST API**: Fast HTTP server using Axum
- **WebSocket Support**: Real-time bidirectional communication

## Building

### Prerequisites

- **Rust 1.70+** ([install from rustup.rs](https://rustup.rs/))
- **Cargo** (comes with Rust)
- **Platform-specific build tools:**
  - **Windows**: Visual Studio Build Tools or Visual Studio with C++ support
  - **Linux**: `build-essential` package (gcc, make, etc.)
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)

### Build

```bash
cd aiOS-rust
cargo build --release
```

### Run

```bash
cargo run --release
```

Or run the release binary:
```bash
./target/release/aios
```

## Architecture

```
aiOS Core (Rust)
├── Action Engine      - Native system calls for actions
├── Vision System      - Screen capture and analysis
├── State Manager      - System state tracking
├── Task Planner       - Task decomposition
├── Event System       - System event monitoring
├── Memory System      - Persistent memory storage
└── API Server         - HTTP/WebSocket interface
```

## API

Same REST API as Python version, but with better performance:

```bash
# Status
curl http://localhost:8765/api/status

# Execute action
curl -X POST http://localhost:8765/api/action \
  -H "Content-Type: application/json" \
  -d '{"action_type": "click", "params": {"x": 500, "y": 300}}'

# Screenshot
curl http://localhost:8765/api/vision/screenshot
```

## Performance Benefits

- **Lower Latency**: Direct system calls, no interpreter overhead
- **Lower Memory**: No garbage collection, efficient memory usage
- **Better Concurrency**: True parallelism with Rust's async model
- **System Access**: Can access kernel-level APIs where permitted

## Platform Support

- **Windows**: Full support via Windows API
- **Linux**: Full support via system calls
- **macOS**: Full support via Cocoa/AppKit

## Development Status

This is a foundational implementation. Core architecture is in place with:
- ✅ Module structure
- ✅ ✅ API Server framework
- ✅ Action Engine structure
- ✅ State management foundation

**TODO**: Implement full functionality for:
- Mouse/keyboard control
- Window management
- Screenshot with cursor reticle
- File operations
- Process management
- Vision analysis
- Task planning

## Comparison with Python Version

| Feature | Python Version | Rust Version |
|---------|---------------|--------------|
| Performance | Good | Excellent |
| Memory Usage | Higher | Lower |
| System Access | Via libraries | Direct APIs |
| Kernel Ops | Limited | Full potential |
| Development Speed | Faster | Slower |
| Runtime Safety | Runtime checks | Compile-time |

## Contributing

This is an ambitious project. Contributions welcome for:
- Platform-specific implementations
- Vision processing
- Task planning algorithms
- Kernel-level operations
- Performance optimizations

## License

[Specify license]

