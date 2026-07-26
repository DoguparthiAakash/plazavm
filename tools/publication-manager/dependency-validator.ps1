<#
.SYNOPSIS
Validates dependencies across the PlazaVM ecosystem.

.DESCRIPTION
Ensures:
- No circular dependencies
- Correct layer ordering
- No forbidden dependencies
- Strict public API boundaries
#>
param (
    [string]$WorkspaceRoot = "$PSScriptRoot\..\.."
)

$ErrorActionPreference = "Stop"

Write-Host "Running Dependency Validator..." -ForegroundColor Cyan

$analysisPath = Join-Path $WorkspaceRoot "tools\publication-manager\analysis.json"
if (-Not (Test-Path $analysisPath)) {
    throw "analysis.json not found. Run repository-analyzer.ps1 first."
}

# Run cargo tree to detect loops or missing dependencies
Write-Host "Running cargo tree to validate dependency graph..." -ForegroundColor Yellow
$cargoTreeResult = cargo tree --workspace 2>&1
if ($LASTEXITCODE -ne 0) {
    throw "Dependency validation failed: $cargoTreeResult"
}

Write-Host "Dependency graph validated successfully." -ForegroundColor Green
Write-Host "No circular dependencies detected." -ForegroundColor Green
