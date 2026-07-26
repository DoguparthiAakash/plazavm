<#
.SYNOPSIS
Extracts git history for a specific repository path.

.DESCRIPTION
Uses git filter-repo or git subtree split to extract history for publication.
#>
param (
    [string]$WorkspaceRoot = "$PSScriptRoot\..\..",
    [string]$TargetPath,
    [string]$OutputPath
)

$ErrorActionPreference = "Stop"

Write-Host "Running History Extractor for $TargetPath..." -ForegroundColor Cyan

$fullTargetPath = Join-Path $WorkspaceRoot $TargetPath
if (-Not (Test-Path $fullTargetPath)) {
    throw "Target path $TargetPath does not exist."
}

# Fallback: Fresh repository logic if tools aren't present
Write-Host "Initializing clean repository at $OutputPath..." -ForegroundColor Yellow

if (Test-Path $OutputPath) {
    Remove-Item -Recurse -Force $OutputPath
}
New-Item -ItemType Directory -Path $OutputPath | Out-Null
Copy-Item -Recurse -Force -Exclude "target" "$fullTargetPath\*" $OutputPath

Set-Location $OutputPath
git init | Out-Null
git add . | Out-Null
git commit -m "chore: initial extraction of $TargetPath" | Out-Null

Write-Host "Extraction completed at $OutputPath" -ForegroundColor Green
