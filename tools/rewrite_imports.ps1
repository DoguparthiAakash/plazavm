$ErrorActionPreference = "Stop"
$Root = "e:\plazavm"

# 1. Remove legacy crates from root Cargo.toml members
$RootCargo = "$Root\Cargo.toml"
$content = Get-Content $RootCargo -Raw
$content = $content -replace '(?m)^\s*"plaza-core",\r?\n?', ''
$content = $content -replace '(?m)^\s*"plaza-events",\r?\n?', ''
$content = $content -replace '(?m)^\s*"plaza-config",\r?\n?', ''
$content = $content -replace '(?m)^\s*"plaza-platform",\r?\n?', ''
Set-Content -Path $RootCargo -Value $content

# 2. Update all other Cargo.tomls to remove legacy dependencies
$TomlFiles = Get-ChildItem -Path $Root -Filter "Cargo.toml" -Recurse | Where-Object { $_.FullName -notmatch "\\target\\" }
foreach ($file in $TomlFiles) {
    if ($file.FullName -eq $RootCargo) { continue }
    $content = Get-Content $file.FullName -Raw
    $changed = $false
    
    if ($content -match 'plaza-(core|events|config|platform)') {
        $content = $content -replace '(?m)^plaza-core\.workspace\s*=\s*true\r?\n?', ''
        $content = $content -replace '(?m)^plaza-events\.workspace\s*=\s*true\r?\n?', ''
        $content = $content -replace '(?m)^plaza-config\.workspace\s*=\s*true\r?\n?', ''
        $content = $content -replace '(?m)^plaza-platform\.workspace\s*=\s*true\r?\n?', ''
        $changed = $true
    }
    
    # Ensure they have plaza-foundation if they had the others
    if ($changed -and $content -notmatch 'plaza-foundation\.workspace') {
        $content = $content -replace '(?m)^(\[dependencies\]\r?\n)', "`$1plaza-foundation.workspace = true`n"
    }

    if ($changed) {
        Set-Content -Path $file.FullName -Value $content
    }
}

# 3. Rewrite all Rust imports
$RsFiles = Get-ChildItem -Path $Root -Filter "*.rs" -Recurse | Where-Object { $_.FullName -notmatch "\\target\\" -and $_.FullName -notmatch "\\plaza-(core|events|config|platform)\\" }
foreach ($file in $RsFiles) {
    $content = Get-Content $file.FullName -Raw
    
    $content = $content -replace '\bplaza_core::', 'plaza_foundation::core::'
    $content = $content -replace '\bplaza_events::', 'plaza_foundation::events::'
    $content = $content -replace '\bplaza_config::', 'plaza_foundation::config::'
    $content = $content -replace '\bplaza_platform::', 'plaza_foundation::platform::'
    
    Set-Content -Path $file.FullName -Value $content
}

# 4. Delete the legacy directories
Remove-Item "$Root\plaza-core" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item "$Root\plaza-events" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item "$Root\plaza-config" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item "$Root\plaza-platform" -Recurse -Force -ErrorAction SilentlyContinue

Write-Host "Workspace rewritten."
