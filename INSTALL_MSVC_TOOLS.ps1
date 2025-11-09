# Install MSVC Build Tools for Rust on Windows
# This script helps set up the Visual Studio Build Tools needed for Rust compilation

Write-Host "=== digiOS MSVC Build Tools Setup ===" -ForegroundColor Cyan
Write-Host ""

# Check if already installed
$vsPath = Get-ChildItem -Path "$env:ProgramFiles" -Directory -Filter "*Visual Studio*" -ErrorAction SilentlyContinue | 
    Where-Object { Test-Path (Join-Path $_.FullName "VC\Tools\MSVC") } | 
    Select-Object -First 1

if ($vsPath) {
    Write-Host "Visual Studio Build Tools found at: $($vsPath.FullName)" -ForegroundColor Green
    Write-Host "MSVC tools should be available!" -ForegroundColor Green
    exit 0
}

Write-Host "MSVC Build Tools not found. Installation required." -ForegroundColor Yellow
Write-Host ""
Write-Host "To install MSVC Build Tools:" -ForegroundColor Cyan
Write-Host "1. Download Visual Studio Build Tools 2022:" -ForegroundColor White
Write-Host "   https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022" -ForegroundColor Blue
Write-Host ""
Write-Host "2. Run the installer (vs_BuildTools.exe)" -ForegroundColor White
Write-Host ""
Write-Host "3. Select 'Desktop development with C++' workload" -ForegroundColor White
Write-Host ""
Write-Host "4. Click Install and wait for completion" -ForegroundColor White
Write-Host ""
Write-Host "5. Restart your terminal/PowerShell after installation" -ForegroundColor White
Write-Host ""
Write-Host "Alternative: Use GNU toolchain instead (no MSVC needed)" -ForegroundColor Yellow
Write-Host "  Run: rustup toolchain install stable-x86_64-pc-windows-gnu" -ForegroundColor White
Write-Host "  Then: rustup default stable-x86_64-pc-windows-gnu" -ForegroundColor White
Write-Host ""

$install = Read-Host "Would you like to open the download page? (Y/N)"
if ($install -eq "Y" -or $install -eq "y") {
    Start-Process "https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022"
}

