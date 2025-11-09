# digiOS Interaction System

## Overview

digiOS includes a comprehensive interaction system that:
- **Automatically detects and installs needed tools**
- **Adapts Windows/Mac programs for use**
- **Provides multiple interaction interfaces**
- **Manages compatibility layers**

## Interaction Interfaces

### 1. Terminal Interface
- Command-line interface for human interaction
- Enabled with `DIGIOS_HUMAN_INTERFACE=1`
- Commands: `help`, `status`, `capabilities`, `exit`

### 2. Web Interface (Future)
- Browser-based UI
- Real-time system monitoring
- Configuration management

### 3. Voice Interface (Future)
- Speech recognition
- Voice commands
- Natural language interaction

## Automatic Tool Management

### How It Works

On boot, digiOS automatically:
1. **Scans for required tools** (Python, Node.js, Git, Cargo, etc.)
2. **Checks if tools are available** in system PATH
3. **Attempts to install missing tools** using:
   - Windows: Chocolatey, winget, Scoop
   - Linux: apt, yum, pacman
   - macOS: Homebrew
4. **Downloads tools** if package managers aren't available
5. **Builds from source** if needed

### Registered Tools

- **Python** - For running Python scripts
- **Node.js** - For web tools and JavaScript
- **Git** - For cloning repositories
- **Cargo** - For Rust compilation (required for self-improvement)

### Adding New Tools

Tools can be registered in `ToolManager`:
```rust
tool_manager.tools.insert("tool_name".to_string(), Tool {
    name: "tool_name".to_string(),
    description: "Description".to_string(),
    platform: vec!["windows".to_string(), "linux".to_string()],
    install_method: InstallMethod::SystemDefault,
    check_command: Some("tool_name --version".to_string()),
    required: false,
});
```

## Compatibility Adapters

### Windows/Mac Program Adaptation

digiOS can adapt and run programs from other platforms:

#### Supported Formats

**Executables:**
- `.exe` - Windows executables (direct execution)
- `.app` - macOS app bundles
- `.msi` - Windows installers
- `.dmg` - macOS disk images

**Scripts:**
- `.py` - Python scripts (runs with Python interpreter)
- `.js` - JavaScript files (runs with Node.js)
- `.sh` - Shell scripts
- `.bat` / `.cmd` - Windows batch files
- `.ps1` - PowerShell scripts

#### How It Works

1. **Detection**: Checks file extension and type
2. **Adaptation**: Determines how to run it
3. **Execution**: Runs with appropriate interpreter or directly
4. **Integration**: Results can be used by digiOS

#### Example Usage

```rust
let adapter = CompatibilityAdapter::new();

// Check if can adapt
if adapter.can_adapt("program.exe").await {
    // Adapt the program
    let adapted = adapter.adapt_program("program.exe").await?;
    
    // Run it
    let output = adapter.run_adapted(&adapted, &["arg1", "arg2"]).await?;
}
```

### File Format Conversion

digiOS can convert files between formats:
- Documents (PDF, DOCX, etc.)
- Media files
- Data formats
- Proprietary formats

## Boot Sequence Integration

The interaction system is integrated into the boot sequence:

```
Boot → Init System
    ↓
Phase 1: System Initialization
    ↓
Phase 2: Tool Setup (NEW!)
    - Check for required tools
    - Install missing tools automatically
    ↓
Phase 3: Model Setup
    ↓
Phase 4: Core Services
    ↓
Phase 5: Interaction System (NEW!)
    - Initialize interaction manager
    - Start terminal if enabled
    - Set up compatibility adapters
    ↓
Phase 6: Self-Improvement
    ↓
Phase 7: Recursive Building
```

## Configuration

### Enable Human Interface

Set environment variable:
```bash
DIGIOS_HUMAN_INTERFACE=1
```

Or in config file:
```json
{
  "interaction": {
    "human_interface": true,
    "auto_tool_install": true
  }
}
```

### Tool Installation

Tools are automatically installed on boot if:
- Tool is marked as `required: true`
- Tool is not found in system PATH
- Package manager is available

## Examples

### Running a Windows Program

```rust
let adapter = CompatibilityAdapter::new();
let program = adapter.adapt_program("C:\\Programs\\app.exe").await?;
let result = adapter.run_adapted(&program, &[]).await?;
```

### Running a Python Script

```rust
let adapter = CompatibilityAdapter::new();
let script = adapter.adapt_program("script.py").await?;
let output = adapter.run_adapted(&script, &["arg1"]).await?;
```

### Ensuring a Tool is Available

```rust
let mut tool_manager = ToolManager::new();
tool_manager.ensure_tool("python").await?;
// Python is now available for use
```

## Future Enhancements

- [ ] Web interface implementation
- [ ] Voice interface implementation
- [ ] More file format conversions
- [ ] Virtual machine integration for cross-platform programs
- [ ] Container support for isolated execution
- [ ] Automatic tool version management
- [ ] Tool dependency resolution

