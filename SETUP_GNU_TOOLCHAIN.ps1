# Alternative: Setup GNU toolchain (no MSVC required)
# This uses MinGW instead of MSVC linker

Write-Host "=== Setting up GNU toolchain for Rust ===" -ForegroundColor Cyan
Write-Host ""

$env:PATH = "C:\Program Files\Rust stable MSVC 1.90\bin;$env:PATH"

Write-Host "Installing GNU toolchain..." -ForegroundColor Yellow
& cargo --version
& rustup toolchain install stable-x86_64-pc-windows-gnu

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "Setting GNU toolchain as default..." -ForegroundColor Yellow
    & rustup default stable-x86_64-pc-windows-gnu
    
    Write-Host ""
    Write-Host "GNU toolchain installed successfully!" -ForegroundColor Green
    Write-Host "You can now build digiOS without MSVC tools." -ForegroundColor Green
    Write-Host ""
    Write-Host "Note: You may need to install MinGW-w64 separately if not already installed." -ForegroundColor Yellow
    Write-Host "Download from: https://www.mingw-w64.org/downloads/" -ForegroundColor Blue
} else {
    Write-Host "Failed to install GNU toolchain. Please check Rust installation." -ForegroundColor Red
}

