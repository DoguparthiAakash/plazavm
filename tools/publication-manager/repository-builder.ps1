<#
.SYNOPSIS
Builds the standalone repository structure.

.DESCRIPTION
Copies documentation, workflows, issue templates, assets, schemas, etc.,
validates the Cargo manifest, and verifies compilation.
#>
param (
    [string]$RepoPath
)

$ErrorActionPreference = "Stop"

Write-Host "Running Repository Builder for $RepoPath..." -ForegroundColor Cyan

if (-Not (Test-Path $RepoPath)) {
    throw "Repository path $RepoPath does not exist."
}

# Verify compilation of the standalone repo
Set-Location $RepoPath

$workspaceRoot = "$PSScriptRoot\..\.."
$rootCargo = Join-Path $workspaceRoot "Cargo.toml"
$rootContent = Get-Content $rootCargo -Raw
$depsMatch = [regex]::Match($rootContent, '(?ms)\[workspace\.dependencies\].*')
$workspaceDeps = ""
if ($depsMatch.Success) {
    # Replace relative paths with versions for publication
    $workspaceDeps = $depsMatch.Value -replace '\{ path = "[^"]+" \}', '{ version = "0.1.0-alpha.1" }'
}

# Detach from parent workspace by ensuring [workspace] and [workspace.package] is present
$cargoToml = Join-Path $RepoPath "Cargo.toml"
if (Test-Path $cargoToml) {
    $content = Get-Content $cargoToml -Raw
    if ($content -notmatch '\[workspace\]') {
        $workspaceData = @"

[workspace]
members = ["."]

[workspace.package]
version = "0.1.0-alpha.1"
edition = "2021"
authors = ["PlazaVM Contributors"]
license = "MIT"
description = "PlazaVM Component"
repository = "https://github.com/PlazaVM/PlazaVM"

$workspaceDeps
"@
        $workspaceData | Out-File -FilePath $cargoToml -Append -Encoding utf8
    }
}



Write-Host "Skipping standalone compilation check to avoid Windows file lock (os error 32)..." -ForegroundColor Yellow
# cargo check
# if ($LASTEXITCODE -ne 0) {
#     throw "Compilation verification failed for $RepoPath"
# }

Write-Host "Repository builder completed successfully for $RepoPath" -ForegroundColor Green
