# Quick Ollama Installation Script for digiOS

Write-Host "=== Ollama Installation for digiOS ===" -ForegroundColor Cyan
Write-Host ""

# Check if already installed
$ollamaPath = "$env:LOCALAPPDATA\Programs\Ollama\ollama.exe"
if (Test-Path $ollamaPath) {
    Write-Host "✓ Ollama found at: $ollamaPath" -ForegroundColor Green
    Write-Host ""
    Write-Host "To start Ollama:" -ForegroundColor Yellow
    Write-Host "  & '$ollamaPath' serve" -ForegroundColor White
    Write-Host ""
    Write-Host "Or it may already be running. Check with:" -ForegroundColor Yellow
    Write-Host "  Test-NetConnection localhost -Port 11434" -ForegroundColor White
    exit 0
}

Write-Host "Ollama server not found." -ForegroundColor Yellow
Write-Host ""
Write-Host "To install Ollama:" -ForegroundColor Cyan
Write-Host "1. Download from: https://ollama.com/download/windows" -ForegroundColor White
Write-Host "2. Run the installer" -ForegroundColor White
Write-Host "3. Restart your terminal" -ForegroundColor White
Write-Host "4. Pull a model: ollama pull llama3.2" -ForegroundColor White
Write-Host "5. Run digiOS - it will detect your models!" -ForegroundColor White
Write-Host ""

$install = Read-Host "Would you like to open the download page? (Y/N)"
if ($install -eq "Y" -or $install -eq "y") {
    Start-Process "https://ollama.com/download/windows"
}

Write-Host ""
Write-Host "Alternative: Use LM Studio or other model runners" -ForegroundColor Yellow
Write-Host "digiOS can detect and use models from:" -ForegroundColor White
Write-Host "  - LM Studio" -ForegroundColor Gray
Write-Host "  - Hugging Face cache" -ForegroundColor Gray
Write-Host "  - Local model files" -ForegroundColor Gray

