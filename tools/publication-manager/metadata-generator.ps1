<#
.SYNOPSIS
Generates GitHub and standard metadata for the repository.

.DESCRIPTION
Generates CI, PR templates, CODEOWNERS, SECURITY.md, etc.
#>
param (
    [string]$RepoPath,
    [string]$TemplatesPath = "$PSScriptRoot\..\templates"
)

$ErrorActionPreference = "Stop"

Write-Host "Running Metadata Generator for $RepoPath..." -ForegroundColor Cyan

if (Test-Path $TemplatesPath) {
    Copy-Item -Recurse -Force "$TemplatesPath\*" $RepoPath
    Write-Host "Injected metadata from templates into $RepoPath" -ForegroundColor Green
} else {
    Write-Host "Warning: Templates directory not found at $TemplatesPath" -ForegroundColor Yellow
}
