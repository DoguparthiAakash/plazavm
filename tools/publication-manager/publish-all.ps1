<#
.SYNOPSIS
Master Pipeline Orchestrator for PlazaVM ecosystem publication.

.DESCRIPTION
Executes the publication of repositories in strict dependency order as specified in the MEPD.
#>
param (
    [string]$WorkspaceRoot = "$PSScriptRoot\..\..",
    [string]$StagingDir = "$PSScriptRoot\..\..\staging"
)

$ErrorActionPreference = "Stop"

$stages = @(
    @("shared/plaza-foundation", "shared/plaza-sdk", "specifications/plaza-specifications", "docs/plaza-docs"),
    @("shared/plaza-command", "engines/plaza-storage", "engines/plaza-workspace", "engines/plaza-runtime"),
    @("engines/plaza-image", "engines/plaza-package", "engines/plaza-plugin", "engines/plaza-registry"),
    @("platform/plaza-security", "platform/plaza-network", "platform/plaza-resource", "platform/plaza-api", "platform/plaza-cloud", "platform/plaza-ai"),
    @("os/plaza-os", "os/plaza-init", "os/plaza-agent", "os/plaza-kernel"),
    @("apps/plaza-cli", "apps/plaza-desktop/src-tauri", "os/plaza-installer")
)

# Step 1: Analyze and Validate Monorepo
Write-Host "=========================================" -ForegroundColor Magenta
Write-Host "STEP 1: ANALYZE AND VALIDATE DEPENDENCIES" -ForegroundColor Magenta
Write-Host "=========================================" -ForegroundColor Magenta
& "$PSScriptRoot\repository-analyzer.ps1" -WorkspaceRoot $WorkspaceRoot
& "$PSScriptRoot\dependency-validator.ps1" -WorkspaceRoot $WorkspaceRoot

if (-Not (Test-Path $StagingDir)) {
    New-Item -ItemType Directory -Path $StagingDir | Out-Null
}

$stageNum = 1
foreach ($stage in $stages) {
    Write-Host "`n=========================================" -ForegroundColor Magenta
    Write-Host "STAGE $stageNum" -ForegroundColor Magenta
    Write-Host "=========================================" -ForegroundColor Magenta

    foreach ($repoPath in $stage) {
        $repoName = Split-Path $repoPath -Leaf
        if ($repoName -eq "src-tauri") {
            $repoName = "plaza-desktop"
        }
        $targetDir = Join-Path $StagingDir $repoName

        Write-Host "Processing $repoName ($repoPath)..." -ForegroundColor Cyan

        # 1. Extract History
        & "$PSScriptRoot\history-extractor.ps1" -WorkspaceRoot $WorkspaceRoot -TargetPath $repoPath -OutputPath $targetDir

        # 2. Inject Metadata and Templates
        & "$PSScriptRoot\metadata-generator.ps1" -RepoPath $targetDir

        # 3. Validate Documentation
        & "$PSScriptRoot\documentation-validator.ps1" -RepoPath $targetDir

        # 4. Assign Versions
        & "$PSScriptRoot\version-manager.ps1" -RepoPath $targetDir

        # 5. Build and Test
        & "$PSScriptRoot\repository-builder.ps1" -RepoPath $targetDir
        & "$PSScriptRoot\quality-runner.ps1" -RepoPath $targetDir

        # 6. Health Check
        & "$PSScriptRoot\health-checker.ps1" -RepoPath $targetDir

        # 7. GitHub Publisher (Creates repo, pushes)
        & "$PSScriptRoot\github-publisher.ps1" -RepoPath $targetDir -RepoName $repoName

        # 8. Release Manager
        & "$PSScriptRoot\release-manager.ps1" -RepoPath $targetDir

        # 9. Publication Reporter
        $reportPath = Join-Path $targetDir "PUBLICATION_REPORT.md"
        & "$PSScriptRoot\publication-reporter.ps1" -RepoPath $targetDir -ReportPath $reportPath
    }
    
    $stageNum++
}

Write-Host "MASTER DIRECTIVE PUBLICATION COMPLETE" -ForegroundColor Green
