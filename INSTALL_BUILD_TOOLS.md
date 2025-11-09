# Installing Build Tools for digiOS

## Quick Installation Options

### Option 1: Visual Studio Build Tools (Recommended)

1. **Download Visual Studio Build Tools:**
   - Go to: https://visualstudio.microsoft.com/downloads/
   - Download "Build Tools for Visual Studio"
   - Run installer

2. **Select Workloads:**
   - Check "Desktop development with C++"
   - This includes MSVC compiler and linker

3. **Install:**
   - Click Install
   - Wait for installation (can take 10-20 minutes)

### Option 2: Use GNU Toolchain (Alternative)

If you prefer not to install Visual Studio, you can use the GNU toolchain:

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

Then build with:
```bash
cargo build --release --target x86_64-pc-windows-gnu
```

### Option 3: Use Chocolatey (Fastest)

If you have Chocolatey installed:

```powershell
choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools"
```

Or just the C++ build tools:
```powershell
choco install visualcppbuildtools
```

### Option 4: Use winget (Windows Package Manager)

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --add Microsoft.VisualStudio.Workload.VCTools --quiet"
```

## Verify Installation

After installation, verify it works:

```bash
cd aiOS-rust
cargo check
```

If it compiles without linker errors, you're good to go!

## Alternative: Use Pre-built Binary

If you just want to test without building, you can:
1. Use a Linux VM or WSL to build
2. Use GitHub Actions to build (CI will build automatically)
3. Wait for releases

## Quick Test Script

Run this to check if tools are available:

```powershell
# Check for MSVC
if (Get-Command cl -ErrorAction SilentlyContinue) {
    Write-Host "MSVC found!" -ForegroundColor Green
} else {
    Write-Host "MSVC not found. Install Visual Studio Build Tools." -ForegroundColor Yellow
}

# Check for Rust
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "Rust/Cargo found!" -ForegroundColor Green
} else {
    Write-Host "Install Rust from rustup.rs" -ForegroundColor Yellow
}
```

