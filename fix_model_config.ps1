# Fix model.json with correct Windows paths
$modelConfigPath = "E:\digiOS\etc\digios\model.json"

if (Test-Path $modelConfigPath) {
    Write-Host "Fixing model.json with correct Windows paths..." -ForegroundColor Yellow
    
    # Read and parse existing config
    $content = Get-Content $modelConfigPath -Raw
    $config = $content | ConvertFrom-Json
    
    # Update with correct Windows path
    $config.local_path = "E:\digiOS\var\lib\digios\models"
    
    # Write back without BOM
    $json = $config | ConvertTo-Json -Depth 10
    [System.IO.File]::WriteAllText($modelConfigPath, $json, [System.Text.UTF8Encoding]::new($false))
    
    Write-Host "Fixed! model.json now has correct Windows paths." -ForegroundColor Green
    Write-Host "Content:" -ForegroundColor Cyan
    Get-Content $modelConfigPath
} else {
    Write-Host "model.json not found. It will be created on first run." -ForegroundColor Yellow
}

