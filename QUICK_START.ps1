# digiOS Quick Start Script
# Sets up environment and checks build readiness

Write-Host "=== digiOS Quick Start ===" -ForegroundColor Cyan
Write-Host ""

# Add Rust to PATH
$rustPath = "C:\Program Files\Rust stable MSVC 1.90\bin"
if (Test-Path $rustPath) {
    $env:PATH = "$rustPath;$env:PATH"
    Write-Host "✓ Rust found at: $rustPath" -ForegroundColor Green
} else {
    Write-Host "✗ Rust not found at expected location" -ForegroundColor Red
    exit 1
}

# Check Rust version
Write-Host ""
Write-Host "Checking Rust installation..." -ForegroundColor Yellow
$rustVersion = & cargo --version 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ $rustVersion" -ForegroundColor Green
} else {
    Write-Host "✗ Cargo not working" -ForegroundColor Red
    exit 1
}

# Check for MSVC tools
Write-Host ""
Write-Host "Checking for MSVC Build Tools..." -ForegroundColor Yellow
$clFound = Get-Command cl.exe -ErrorAction SilentlyContinue
$linkFound = Get-Command link.exe -ErrorAction SilentlyContinue

if ($clFound -and $linkFound) {
    Write-Host "✓ MSVC tools found!" -ForegroundColor Green
    Write-Host "  cl.exe: $($clFound.Source)" -ForegroundColor Gray
    Write-Host "  link.exe: $($linkFound.Source)" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Ready to build! Run: cargo build --release" -ForegroundColor Green
} else {
    Write-Host "✗ MSVC Build Tools not found" -ForegroundColor Red
    Write-Host ""
    Write-Host "To install MSVC Build Tools:" -ForegroundColor Yellow
    Write-Host "  1. Run: .\INSTALL_MSVC_TOOLS.ps1" -ForegroundColor White
    Write-Host "  2. Or download from:" -ForegroundColor White
    Write-Host "     https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022" -ForegroundColor Blue
    Write-Host ""
    Write-Host "Alternative: Use GNU toolchain (no MSVC needed)" -ForegroundColor Yellow
    Write-Host "  Note: Your Rust installation doesn't include rustup," -ForegroundColor Gray
    Write-Host "  so you'll need to install rustup separately or use MSVC tools." -ForegroundColor Gray
}

# Try a quick check
Write-Host ""
Write-Host "Testing compilation..." -ForegroundColor Yellow
cd $PSScriptRoot
$checkResult = & cargo check 2>&1 | Select-Object -Last 10

if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Compilation check passed!" -ForegroundColor Green
} else {
    Write-Host "✗ Compilation check failed" -ForegroundColor Red
    Write-Host ""
    Write-Host "Last errors:" -ForegroundColor Yellow
    $checkResult | ForEach-Object { Write-Host $_ -ForegroundColor Red }
}

