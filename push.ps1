param (
    [Parameter(Mandatory=$true, HelpMessage="Name of the repository to push (e.g. plaza-foundation)")]
    [string]$Repository,
    [switch]$Force
)

$StagingDir = "$PSScriptRoot\staging"
$repoPath = Join-Path $StagingDir $Repository

if (-not (Test-Path $repoPath)) {
    Write-Host "Repository '$Repository' not found in staging area ($repoPath)." -ForegroundColor Red
    exit 1
}

Write-Host "=========================================" -ForegroundColor Magenta
Write-Host " PUSHING $Repository TO GITHUB" -ForegroundColor Magenta
Write-Host "=========================================" -ForegroundColor Magenta

Set-Location $repoPath

$pushArgs = @("push", "origin", "HEAD:main")
if ($Force) {
    $pushArgs += "--force"
}

git @pushArgs

if ($LASTEXITCODE -eq 0) {
    Write-Host "`nSuccessfully pushed $Repository" -ForegroundColor Green
} else {
    Write-Host "`nFailed to push $Repository" -ForegroundColor Red
}

Set-Location $PSScriptRoot
