# Create Bootable digiOS on E:\ Drive
# Run this script to set up digiOS for boot testing

Write-Host "Creating bootable digiOS on E:\ drive..." -ForegroundColor Green

# Check if E: exists
if (-not (Test-Path "E:\")) {
    Write-Host "ERROR: E:\ drive not found!" -ForegroundColor Red
    exit 1
}

# Create directory structure
Write-Host "Creating directory structure..." -ForegroundColor Yellow
$dirs = @(
    "E:\digiOS",
    "E:\digiOS\boot",
    "E:\digiOS\etc\digios",
    "E:\digiOS\var\lib\digios\models",
    "E:\digiOS\var\lib\digios\memory",
    "E:\digiOS\bin",
    "E:\digiOS\sbin"
)

foreach ($dir in $dirs) {
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
}

# Build the release binary
Write-Host "Building release binary..." -ForegroundColor Yellow
$buildResult = cargo build --release 2>&1

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed. Attempting to continue with existing binary..." -ForegroundColor Yellow
}

# Copy binary
$binaryPath = "target\release\aios.exe"
if (Test-Path $binaryPath) {
    Write-Host "Copying binary to E:\digiOS\sbin\digios.exe..." -ForegroundColor Yellow
    Copy-Item $binaryPath "E:\digiOS\sbin\digios.exe" -Force
} else {
    Write-Host "WARNING: Binary not found at $binaryPath" -ForegroundColor Yellow
    Write-Host "You may need to build manually: cargo build --release" -ForegroundColor Yellow
}

# Create default config
Write-Host "Creating default configuration..." -ForegroundColor Yellow
$config = @{
    api = @{
        port = 8765
        host = "0.0.0.0"
    }
    memory = @{
        path = "/var/lib/digios/memory"
        max_history = 10000
    }
    system = @{
        safety_mode = $true
        enable_kernel_ops = $false
        max_concurrent_tasks = 10
    }
    features = @{
        vision = $true
        ocr = $true
        object_detection = $true
        event_monitoring = $true
        memory_persistence = $true
    }
} | ConvertTo-Json -Depth 10

$config | Out-File -FilePath "E:\digiOS\etc\digios\config.json" -Encoding UTF8

# Create model config
$modelConfig = @{
    name = "digios-default"
    size = "Medium"
    url = $null
    local_path = "/var/lib/digios/models"
} | ConvertTo-Json

$modelConfig | Out-File -FilePath "E:\digiOS\etc\digios\model.json" -Encoding UTF8

# Create GRUB configuration
Write-Host "Creating GRUB configuration..." -ForegroundColor Yellow
$grubConfig = @"
menuentry "digiOS" {
    linux /digiOS/sbin/digios.exe
    initrd /digiOS/boot/initrd.img
}
"@

$grubConfig | Out-File -FilePath "E:\digiOS\boot\grub.cfg" -Encoding UTF8

# Create init script
Write-Host "Creating init script..." -ForegroundColor Yellow
$initScript = @"
#!/bin/sh
# digiOS Init Script
cd /digiOS
/sbin/digios.exe
"@

$initScript | Out-File -FilePath "E:\digiOS\boot\init" -Encoding UTF8

# Create README
$readme = @"
digiOS Bootable System
=====================

This is a bootable digiOS installation on E:\ drive.

Directory Structure:
- /boot/          - Boot files and GRUB config
- /etc/digios/    - Configuration files
- /var/lib/digios/models/  - AI models storage
- /var/lib/digios/memory/  - System memory
- /sbin/          - System binaries (digios.exe)

To boot:
1. Configure your bootloader to boot from E:\digiOS
2. Or use a boot manager to chainload
3. Or create a bootable USB/CD from this structure

First Boot:
- System will run setup wizard
- Will download AI model
- Will begin recursive self-improvement

For testing without booting:
- Run: E:\digiOS\sbin\digios.exe
- Or set DIGIOS_HUMAN_INTERFACE=1 for terminal interface
"@

$readme | Out-File -FilePath "E:\digiOS\README.txt" -Encoding UTF8

Write-Host "`nBootable digiOS created on E:\digiOS" -ForegroundColor Green
Write-Host "Binary location: E:\digiOS\sbin\digios.exe" -ForegroundColor Cyan
Write-Host "Config location: E:\digiOS\etc\digios\config.json" -ForegroundColor Cyan
Write-Host "`nTo test without booting, run: E:\digiOS\sbin\digios.exe" -ForegroundColor Yellow

