$ErrorActionPreference = "Stop"

$workspaceRoot = "E:\plazavm"
Write-Host "Committing all changes in PlazaVM repositories..."

# Array of all sub-repositories (plugins, platform, engines, shared, apps)
$repos = @(
    "$workspaceRoot\apps\plaza-cli",
    "$workspaceRoot\apps\plaza-desktop",
    "$workspaceRoot\apps\plaza-installer",
    "$workspaceRoot\engines\plaza-image",
    "$workspaceRoot\engines\plaza-registry",
    "$workspaceRoot\engines\plaza-runtime",
    "$workspaceRoot\engines\plaza-workspace",
    "$workspaceRoot\platform\plaza-api",
    "$workspaceRoot\platform\plaza-docs",
    "$workspaceRoot\platform\plaza-kernel",
    "$workspaceRoot\platform\plaza-network",
    "$workspaceRoot\platform\plaza-os",
    "$workspaceRoot\platform\plaza-package",
    "$workspaceRoot\platform\plaza-specifications",
    "$workspaceRoot\plugins\docker",
    "$workspaceRoot\plugins\hyperv",
    "$workspaceRoot\plugins\podman",
    "$workspaceRoot\plugins\qemu",
    "$workspaceRoot\plugins\v86",
    "$workspaceRoot\plugins\virtualbox",
    "$workspaceRoot\shared\plaza-command",
    "$workspaceRoot\shared\plaza-foundation",
    "$workspaceRoot\shared\plaza-storage"
)

# Function to commit a repository
function Commit-Repo {
    param([string]$repoPath)
    
    if (Test-Path "$repoPath\.git") {
        Write-Host "Checking repository: $repoPath"
        Set-Location $repoPath
        
        $status = git status --porcelain
        if ($status) {
            Write-Host "  Changes detected. Committing..." -ForegroundColor Yellow
            git add .
            git commit -m "Updates"
            Write-Host "  Committed." -ForegroundColor Green
        } else {
            Write-Host "  No changes." -ForegroundColor DarkGray
        }
    } else {
        Write-Host "Skipping $repoPath (Not a git repository)" -ForegroundColor DarkGray
    }
}

# Commit each sub-repository
foreach ($repo in $repos) {
    Commit-Repo $repo
}

# Commit the root workspace repository
Write-Host "Checking root repository: $workspaceRoot"
Set-Location $workspaceRoot
$rootStatus = git status --porcelain
if ($rootStatus) {
    Write-Host "  Changes detected in root. Committing..." -ForegroundColor Yellow
    git add .
    git commit -m "Updates"
    Write-Host "  Committed root." -ForegroundColor Green
} else {
    Write-Host "  No changes in root." -ForegroundColor DarkGray
}

Write-Host "Done!"
