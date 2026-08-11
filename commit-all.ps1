$ErrorActionPreference = "Stop"

# Commit the main repository
Write-Host "Committing main repository..."
git add .
git commit -m "Updates"

# Commit all embedded repositories in the staging directory
$stagingDir = ".\staging"
if (Test-Path $stagingDir) {
    $repos = Get-ChildItem -Path $stagingDir -Directory
    foreach ($repo in $repos) {
        $repoPath = $repo.FullName
        # Check if it's a git repository
        if (Test-Path "$repoPath\.git") {
            Write-Host "Committing repository: $($repo.Name)..."
            Set-Location $repoPath
            git add .
            # We use try/catch or ignore errors in case there are no changes to commit
            try {
                git commit -m "Updates" *>&1 | Out-Null
                Write-Host " -> Committed."
            } catch {
                Write-Host " -> No changes or failed to commit."
            }
        }
    }
}

# Return to root
Set-Location $PSScriptRoot
Write-Host "Done."
