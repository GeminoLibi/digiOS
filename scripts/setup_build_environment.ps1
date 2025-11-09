# Complete Build Environment Setup for digiOS
# This script installs both Rust and MSVC build tools

Write-Host "digiOS Complete Build Environment Setup" -ForegroundColor Green
Write-Host "=======================================" -ForegroundColor Green
Write-Host ""

# Check what we need
$needsRust = $false
$needsMSVC = $false

# Check for Rust
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "✓ Rust/Cargo found" -ForegroundColor Green
    cargo --version
} else {
    Write-Host "✗ Rust/Cargo not found - will install" -ForegroundColor Yellow
    $needsRust = $true
}

Write-Host ""

# Check for MSVC
if (Get-Command cl -ErrorAction SilentlyContinue) {
    Write-Host "✓ MSVC compiler found" -ForegroundColor Green
    cl 2>&1 | Select-Object -First 1
} else {
    Write-Host "✗ MSVC compiler not found - will install" -ForegroundColor Yellow
    $needsMSVC = $true
}

Write-Host ""

if (-not $needsRust -and -not $needsMSVC) {
    Write-Host "All tools are already installed!" -ForegroundColor Green
    Write-Host "You can build digiOS now with: cargo build --release" -ForegroundColor Cyan
    exit 0
}

# Install Rust first (if needed)
if ($needsRust) {
    Write-Host "Installing Rust..." -ForegroundColor Yellow
    Write-Host "Downloading rustup-init.exe..." -ForegroundColor Cyan
    
    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupPath = "$env:TEMP\rustup-init.exe"
    
    try {
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath -UseBasicParsing
        Write-Host "Running rustup installer..." -ForegroundColor Cyan
        Write-Host "Please follow the prompts (default options are fine)" -ForegroundColor Yellow
        
        Start-Process -FilePath $rustupPath -Wait -NoNewWindow
        
        # Refresh PATH
        $env:PATH = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
        
        Write-Host "Rust installation complete!" -ForegroundColor Green
        Write-Host "You may need to restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
    } catch {
        Write-Host "Error installing Rust: $_" -ForegroundColor Red
        Write-Host "Manual installation: https://rustup.rs" -ForegroundColor Cyan
    }
}

Write-Host ""

# Install MSVC Build Tools (if needed)
if ($needsMSVC) {
    Write-Host "Installing Visual Studio Build Tools..." -ForegroundColor Yellow
    
    if (Get-Command winget -ErrorAction SilentlyContinue) {
        Write-Host "Using winget to install..." -ForegroundColor Cyan
        Write-Host "This will take 10-20 minutes. Please wait..." -ForegroundColor Yellow
        
        try {
            winget install Microsoft.VisualStudio.2022.BuildTools `
                --override "--wait --add Microsoft.VisualStudio.Workload.VCTools --quiet --norestart" `
                --accept-package-agreements --accept-source-agreements
            
            Write-Host "MSVC Build Tools installation complete!" -ForegroundColor Green
        } catch {
            Write-Host "Error installing via winget: $_" -ForegroundColor Red
            Write-Host "You can manually download from:" -ForegroundColor Cyan
            Write-Host "https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022" -ForegroundColor Cyan
        }
    } else {
        Write-Host "winget not available. Manual installation required:" -ForegroundColor Yellow
        Write-Host "1. Go to: https://visualstudio.microsoft.com/downloads/" -ForegroundColor Cyan
        Write-Host "2. Download 'Build Tools for Visual Studio 2022'" -ForegroundColor Cyan
        Write-Host "3. Run installer and select 'Desktop development with C++'" -ForegroundColor Cyan
        Write-Host "4. Click Install" -ForegroundColor Cyan
    }
}

Write-Host ""
Write-Host "Setup Summary:" -ForegroundColor Green
Write-Host "=============" -ForegroundColor Green
if (-not $needsRust) { Write-Host "✓ Rust already installed" -ForegroundColor Green }
if (-not $needsMSVC) { Write-Host "✓ MSVC already installed" -ForegroundColor Green }

Write-Host ""
Write-Host "Next Steps:" -ForegroundColor Yellow
Write-Host "1. Restart your terminal (to refresh PATH)" -ForegroundColor Cyan
Write-Host "2. Run: cd aiOS-rust" -ForegroundColor Cyan
Write-Host "3. Run: cargo build --release" -ForegroundColor Cyan
Write-Host ""

