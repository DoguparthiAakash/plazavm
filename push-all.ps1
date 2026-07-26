param (
    [switch]$Force
)

$StagingDir = "$PSScriptRoot\staging"

if (-not (Test-Path $StagingDir)) {
    Write-Host "Staging directory not found. Please run .\extract.ps1 first." -ForegroundColor Red
    exit 1
}

$repos = Get-ChildItem -Path $StagingDir -Directory

Write-Host "=========================================" -ForegroundColor Magenta
Write-Host " PUSHING ALL REPOSITORIES TO GITHUB" -ForegroundColor Magenta
Write-Host "=========================================" -ForegroundColor Magenta

foreach ($repo in $repos) {
    Write-Host "`nPushing $($repo.Name)..." -ForegroundColor Cyan
    Set-Location $repo.FullName

    $pushArgs = @("push", "origin", "HEAD:main")
    if ($Force) {
        $pushArgs += "--force"
    }

    git @pushArgs
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Successfully pushed $($repo.Name)" -ForegroundColor Green
    } else {
        Write-Host "Failed to push $($repo.Name)" -ForegroundColor Red
    }
}

Set-Location $PSScriptRoot
Write-Host "`n=========================================" -ForegroundColor Magenta
Write-Host " ALL REPOSITORIES PUSHED" -ForegroundColor Magenta
Write-Host "=========================================" -ForegroundColor Magenta
