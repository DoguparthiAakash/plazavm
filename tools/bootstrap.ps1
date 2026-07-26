$ErrorActionPreference = "Stop"
$ProjectRoot = "e:\plazavm"

Set-Location $ProjectRoot

# 1. Create top-level directories
$TopLevelDirs = @(
    "apps", "engines", "platform", "shared", "os", "specifications", "docs", "scripts", 
    "tools", "cargo", "examples", "tests", "benchmarks", "assets", "templates", "design", 
    "deployments", "infrastructure", ".github", "integration"
)

foreach ($dir in $TopLevelDirs) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir | Out-Null
        Write-Host "Created directory: $dir"
    }
}

# 2. Define Repositories and their categories
$Repositories = @{
    "shared" = @("plaza-foundation", "plaza-command", "plaza-sdk")
    "engines" = @("plaza-workspace", "plaza-runtime", "plaza-storage", "plaza-image", "plaza-package", "plaza-plugin", "plaza-registry", "plaza-snapshot")
    "platform" = @("plaza-security", "plaza-network", "plaza-resource", "plaza-ai", "plaza-api", "plaza-cloud")
    "apps" = @("plaza-cli", "plaza-desktop")
    "os" = @("plaza-os", "plaza-init", "plaza-agent", "plaza-kernel", "plaza-installer")
    "specifications" = @("plaza-specifications")
    "docs" = @("plaza-docs")
    "tools" = @("plaza-build")
    "examples" = @("plaza-examples")
    "integration" = @("plaza-manifest")
}

# 3. Migrate / Scaffold Repositories
$RequiredFiles = @("README.md", "ARCHITECTURE.md", "ROADMAP.md", "DESIGN.md", "API.md", "CHANGELOG.md", "CONTRIBUTING.md", "SECURITY.md", "LICENSE", "CODEOWNERS")
$RequiredDirs = @("docs", "examples", "tests", "benchmarks", ".github", "schemas", "assets", "src")

foreach ($category in $Repositories.Keys) {
    $repos = $Repositories[$category]
    foreach ($repo in $repos) {
        $targetPath = Join-Path $category $repo
        
        # If the crate exists in root, move it
        if (Test-Path $repo -PathType Container) {
            Move-Item -Path $repo -Destination $targetPath -Force
            Write-Host "Moved $repo to $targetPath"
        } elseif (-not (Test-Path $targetPath)) {
            New-Item -ItemType Directory -Path $targetPath | Out-Null
            Write-Host "Scaffolded $targetPath"
        }

        # Scaffold files
        foreach ($file in $RequiredFiles) {
            $filePath = Join-Path $targetPath $file
            if (-not (Test-Path $filePath)) {
                Set-Content -Path $filePath -Value "# $repo - $file`n`nPlaceholder for $file."
            }
        }

        # Scaffold dirs
        foreach ($dir in $RequiredDirs) {
            $dirPath = Join-Path $targetPath $dir
            if (-not (Test-Path $dirPath)) {
                New-Item -ItemType Directory -Path $dirPath | Out-Null
            }
        }

        # Create basic Cargo.toml if missing
        $cargoToml = Join-Path $targetPath "Cargo.toml"
        if (-not (Test-Path $cargoToml)) {
            $cargoContent = @"
[package]
name = `"$repo`"
version = `"0.1.0`"
edition = `"2021`"

[dependencies]
"@
            Set-Content -Path $cargoToml -Value $cargoContent
        }

        # Create basic lib.rs if src is empty
        $libRs = Join-Path $targetPath "src\lib.rs"
        $mainRs = Join-Path $targetPath "src\main.rs"
        if (-not (Test-Path $libRs) -and -not (Test-Path $mainRs)) {
            Set-Content -Path $libRs -Value "pub fn init() {}"
        }
    }
}

Write-Host "Repository scaffolding complete."
