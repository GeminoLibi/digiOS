# Install Build Tools for digiOS
# This script helps install MSVC build tools

Write-Host "digiOS Build Tools Installation Helper" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green
Write-Host ""

# Check current status
Write-Host "Checking current setup..." -ForegroundColor Yellow

$hasMSVC = $false
$hasRust = $false

# Check for MSVC
if (Get-Command cl -ErrorAction SilentlyContinue) {
    Write-Host "✓ MSVC compiler found" -ForegroundColor Green
    $hasMSVC = $true
} else {
    Write-Host "✗ MSVC compiler not found" -ForegroundColor Red
}

# Check for Rust
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "✓ Rust/Cargo found" -ForegroundColor Green
    $hasRust = $true
} else {
    Write-Host "✗ Rust/Cargo not found" -ForegroundColor Red
}

Write-Host ""

if ($hasMSVC -and $hasRust) {
    Write-Host "All build tools are available! You can build digiOS now." -ForegroundColor Green
    Write-Host "Run: cargo build --release" -ForegroundColor Cyan
    exit 0
}

# Installation options
Write-Host "Installation Options:" -ForegroundColor Yellow
Write-Host "1. Use Chocolatey (if installed)"
Write-Host "2. Use winget (Windows 10/11)"
Write-Host "3. Manual download instructions"
Write-Host "4. Use GNU toolchain (alternative)"
Write-Host ""

$choice = Read-Host "Select option [1-4]"

switch ($choice) {
    "1" {
        if (Get-Command choco -ErrorAction SilentlyContinue) {
            Write-Host "Installing Visual C++ Build Tools via Chocolatey..." -ForegroundColor Yellow
            choco install visualcppbuildtools -y
        } else {
            Write-Host "Chocolatey not found. Install from: https://chocolatey.org" -ForegroundColor Red
        }
    }
    "2" {
        if (Get-Command winget -ErrorAction SilentlyContinue) {
            Write-Host "Installing Visual Studio Build Tools via winget..." -ForegroundColor Yellow
            Write-Host "This may take 10-20 minutes..." -ForegroundColor Yellow
            winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --add Microsoft.VisualStudio.Workload.VCTools --quiet"
        } else {
            Write-Host "winget not available on this system" -ForegroundColor Red
        }
    }
    "3" {
        Write-Host ""
        Write-Host "Manual Installation:" -ForegroundColor Yellow
        Write-Host "1. Go to: https://visualstudio.microsoft.com/downloads/" -ForegroundColor Cyan
        Write-Host "2. Download 'Build Tools for Visual Studio'" -ForegroundColor Cyan
        Write-Host "3. Run installer and select 'Desktop development with C++'" -ForegroundColor Cyan
        Write-Host "4. Click Install and wait" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "Or use direct download:" -ForegroundColor Yellow
        Write-Host "https://aka.ms/vs/17/release/vs_buildtools.exe" -ForegroundColor Cyan
    }
    "4" {
        Write-Host "Setting up GNU toolchain..." -ForegroundColor Yellow
        rustup toolchain install stable-x86_64-pc-windows-gnu
        rustup default stable-x86_64-pc-windows-gnu
        Write-Host "GNU toolchain installed. Build with:" -ForegroundColor Green
        Write-Host "  cargo build --release --target x86_64-pc-windows-gnu" -ForegroundColor Cyan
    }
    default {
        Write-Host "Invalid choice" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "After installation, restart your terminal and run:" -ForegroundColor Yellow
Write-Host "  cd aiOS-rust" -ForegroundColor Cyan
Write-Host "  cargo build --release" -ForegroundColor Cyan

