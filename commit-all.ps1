param (
    [string]$CommitMessage = "Updates"
)

# Function to commit in a specific directory
function Commit-Directory {
    param (
        [string]$Path
    )
    
    if (Test-Path "$Path\.git") {
        Write-Host "Committing in repository: $Path" -ForegroundColor Cyan
        Push-Location $Path
        
        # Check if there are changes
        $status = git status --porcelain
        if ([string]::IsNullOrWhiteSpace($status)) {
            Write-Host "No changes to commit in $Path." -ForegroundColor Yellow
        } else {
            git add .
            git commit -m $CommitMessage
            Write-Host "Successfully committed changes in $Path." -ForegroundColor Green
        }
        
        Pop-Location
    }
}

# 1. Commit in all sub-repositories in staging/
$stagingPath = Join-Path $PSScriptRoot "staging"
if (Test-Path $stagingPath) {
    $subRepos = Get-ChildItem -Path $stagingPath -Directory
    foreach ($repo in $subRepos) {
        Commit-Directory -Path $repo.FullName
    }
}

# 2. Commit in the main/common repository (root)
Commit-Directory -Path $PSScriptRoot

Write-Host "All commit operations completed." -ForegroundColor Green
