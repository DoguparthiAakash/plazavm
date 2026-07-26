<#
.SYNOPSIS
Publishes all staged repositories to GitHub as public repositories.

.DESCRIPTION
Iterates through all repositories in the staging directory and uses the GitHub CLI (gh)
to create a public repository under the PlazaVM organization and push the code.
#>
param (
    [string]$StagingDir = "$PSScriptRoot\staging"
)

$ErrorActionPreference = "Stop"

if (-Not (Get-Command "gh" -ErrorAction SilentlyContinue)) {
    throw "GitHub CLI (gh) is not installed or not in PATH. Please install it from https://cli.github.com/"
}

if (-Not (Test-Path $StagingDir)) {
    throw "Staging directory not found at $StagingDir. Please run the publication extraction pipeline first."
}

Write-Host "=========================================" -ForegroundColor Magenta
Write-Host "PUBLISHING ALL REPOSITORIES TO GITHUB" -ForegroundColor Magenta
Write-Host "=========================================" -ForegroundColor Magenta

$repos = Get-ChildItem -Path $StagingDir -Directory

foreach ($repo in $repos) {
    $repoName = $repo.Name
    $repoPath = $repo.FullName

    Write-Host "`nPublishing $OrgName/$repoName..." -ForegroundColor Cyan
    Set-Location $repoPath

    # Check if git is initialized
    if (-Not (Test-Path (Join-Path $repoPath ".git"))) {
        Write-Host "Initializing git repository in $repoName..." -ForegroundColor Yellow
        git init | Out-Null
        git add . | Out-Null
        git commit -m "chore: initial extraction for publication" | Out-Null
    }

    # Create the repository on GitHub and push
    Write-Host "Running: gh repo create $repoName --public --source . --remote origin --push" -ForegroundColor Yellow
    gh repo create "$repoName" --public --source . --remote origin --push
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Successfully published $repoName!" -ForegroundColor Green
    } else {
        Write-Host "Failed to publish $repoName. It may already exist or there may be a network issue." -ForegroundColor Red
    }
}

Write-Host "`n=========================================" -ForegroundColor Magenta
Write-Host "ALL REPOSITORIES PUBLISHED" -ForegroundColor Magenta
Write-Host "=========================================" -ForegroundColor Magenta
