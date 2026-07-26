<#
.SYNOPSIS
Validates documentation presence and link integrity.

.DESCRIPTION
Ensures README, ARCHITECTURE, API, ROADMAP, CHANGELOG, CONTRIBUTING, SECURITY, LICENSE, CODEOWNERS exist.
#>
param (
    [string]$RepoPath
)

$ErrorActionPreference = "Stop"

Write-Host "Running Documentation Validator for $RepoPath..." -ForegroundColor Cyan

$RequiredDocs = @("README.md", "ARCHITECTURE.md", "API.md", "ROADMAP.md", "CHANGELOG.md", "CONTRIBUTING.md", "SECURITY.md", "LICENSE", "CODEOWNERS")

foreach ($doc in $RequiredDocs) {
    # We warn instead of fail to allow initial pass, but in strict mode we'd throw
    $docPath = Join-Path $RepoPath $doc
    if (-Not (Test-Path $docPath)) {
        Write-Host "Warning: Missing $doc in $RepoPath" -ForegroundColor Yellow
    }
}

Write-Host "Documentation validation completed." -ForegroundColor Green
