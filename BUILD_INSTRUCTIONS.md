# Building digiOS on Windows

## Current Status

Rust is installed at: `C:\Program Files\Rust stable MSVC 1.90\bin\`

However, MSVC Build Tools are required to compile Rust code on Windows.

## Option 1: Install MSVC Build Tools (Recommended)

1. **Run the setup script:**
   ```powershell
   .\INSTALL_MSVC_TOOLS.ps1
   ```

2. **Or manually:**
   - Download: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
   - Run `vs_BuildTools.exe`
   - Select "Desktop development with C++" workload
   - Click Install
   - Restart your terminal/PowerShell

3. **Verify installation:**
   ```powershell
   $env:PATH = "C:\Program Files\Rust stable MSVC 1.90\bin;$env:PATH"
   cargo check
   ```

## Option 2: Use GNU Toolchain (Alternative)

If you prefer not to install MSVC tools:

1. **Install MinGW-w64:**
   - Download from: https://www.mingw-w64.org/downloads/
   - Or use: `winget install mingw-w64`

2. **Switch Rust to GNU toolchain:**
   ```powershell
   $env:PATH = "C:\Program Files\Rust stable MSVC 1.90\bin;$env:PATH"
   rustup toolchain install stable-x86_64-pc-windows-gnu
   rustup default stable-x86_64-pc-windows-gnu
   ```

3. **Build:**
   ```powershell
   cargo build --release
   ```

## Building After Setup

Once MSVC or GNU tools are installed:

```powershell
# Navigate to project
cd aiOS-rust

# Add Rust to PATH (if not already)
$env:PATH = "C:\Program Files\Rust stable MSVC 1.90\bin;$env:PATH"

# Check compilation
cargo check

# Build release
cargo build --release

# Run
cargo run --release
```

## Troubleshooting

### Error: `link.exe` not found
- **Solution**: Install MSVC Build Tools (Option 1) or switch to GNU toolchain (Option 2)

### Error: `rustup` not found
- **Solution**: Your Rust installation may not include rustup. Use cargo directly:
  ```powershell
  & "C:\Program Files\Rust stable MSVC 1.90\bin\cargo.exe" build --release
  ```

### Error: `cargo` not found
- **Solution**: Add Rust to PATH:
  ```powershell
  $env:PATH = "C:\Program Files\Rust stable MSVC 1.90\bin;$env:PATH"
  ```

