# digiOS - AI Native Operating System

A bootable, self-improving AI-native operating system built in Rust. digiOS boots from scratch, downloads its AI model, and recursively builds itself into a complete system.

**digiOS** (Digital Intelligence Operating System) is designed to be a true AI-native OS that:
- **Boots independently** - Runs as init system (PID 1)
- **Downloads its model** - Acquires AI capabilities on first boot
- **Recursively builds itself** - Continuously improves and expands capabilities
- **Supports human interaction** - Optional terminal/web/voice interfaces
- **Self-improving** - Uses AI to generate code and enhance itself

## Core Philosophy

digiOS is built on the principle of **recursive self-improvement**. On boot, it:
1. Downloads and loads its AI model
2. Evaluates its current capabilities
3. Identifies areas for improvement
4. Generates code to implement improvements
5. Compiles and integrates new code
6. Repeats the process, building itself into a complete system

This creates a system that starts minimal but grows into a full-featured AI-native operating system.

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

## Boot Process

1. **Bootloader** loads kernel and initramfs
2. **Init System** starts (PID 1)
3. **First Boot Check** - Runs setup wizard if needed
4. **Model Download** - Downloads AI model if not present
5. **Model Loading** - Loads model into memory
6. **Core Initialization** - Starts all core services
7. **Self-Improvement Start** - Begins recursive building
8. **Human Interface** - Starts if enabled

## Self-Improvement Cycle

The system continuously:
1. **Evaluates** its current state and capabilities
2. **Identifies** areas for improvement
3. **Generates** code using the AI model
4. **Compiles** and tests new code
5. **Integrates** improvements into the system
6. **Repeats** the cycle, building itself recursively

## Development Status

### ✅ Completed
- Boot system architecture
- Init system with boot sequence
- Setup wizard for first boot
- Model management system
- Model downloader framework
- Self-improvement engine architecture
- Code generation framework
- System evaluator
- Human interface (terminal)
- Core service architecture

### 🚧 In Progress
- Actual model integration
- Code compilation pipeline
- Hot-reload mechanism
- Bootable image creation

### 📋 TODO
- Mouse/keyboard control implementation
- Window management
- Screenshot with cursor reticle
- File operations
- Process management
- Vision analysis
- Task planning
- Web interface
- Voice interface
- Bootable image builder

## Architecture

```
digiOS System
├── Boot System
│   ├── Init System      - First process (PID 1), boot sequence
│   ├── Setup Wizard     - First-boot configuration
│   └── Bootloader       - Boot configuration
├── Model System
│   ├── Model Manager    - Model lifecycle management
│   ├── Model Downloader - Downloads models on first boot
│   └── Model Loader     - Loads and runs AI models
├── Core Services
│   ├── Action Engine    - Native system calls for actions
│   ├── Vision System    - Screen capture and analysis
│   ├── State Manager    - System state tracking
│   ├── Task Planner     - Task decomposition
│   ├── Event System     - System event monitoring
│   ├── Memory System    - Persistent memory storage
│   └── API Server       - HTTP/WebSocket interface
├── Self-Improvement
│   ├── Improvement Engine - Orchestrates self-improvement
│   ├── Code Generator   - Generates code using AI
│   └── System Evaluator - Evaluates and identifies improvements
└── Human Interface
    ├── Terminal         - Command-line interface
    ├── Web              - Browser-based UI (future)
    └── Voice            - Voice interaction (future)
```

## Performance Benefits

- **Lower Latency**: Direct system calls, no interpreter overhead
- **Lower Memory**: No garbage collection, efficient memory usage
- **Better Concurrency**: True parallelism with Rust's async model
- **System Access**: Can access kernel-level APIs where permitted

## Contributing

This is an ambitious project. Contributions welcome for:
- Platform-specific implementations
- Vision processing
- Task planning algorithms
- Kernel-level operations
- Performance optimizations

## License

[Specify license]

