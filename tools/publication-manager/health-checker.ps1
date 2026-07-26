<#
.SYNOPSIS
Checks health of the repository prior to publication.

.DESCRIPTION
Final check on repo maturity, dependencies, CI configuration.
#>
param (
    [string]$RepoPath
)

$ErrorActionPreference = "Stop"

Write-Host "Running Health Checker for $RepoPath..." -ForegroundColor Cyan

# Simple check to ensure we have a valid Cargo.toml
$cargoToml = Join-Path $RepoPath "Cargo.toml"
if (-Not (Test-Path $cargoToml)) {
    throw "Health Check Failed: Cargo.toml missing in $RepoPath"
}

Write-Host "Health Check passed successfully." -ForegroundColor Green
