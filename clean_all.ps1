# clean_all.ps1
# This script recursively finds all Cargo.toml files and runs `cargo clean` in their directories.

Write-Host "Finding all Rust projects to clean..." -ForegroundColor Cyan
$cargoFiles = Get-ChildItem -Path $PSScriptRoot -Filter "Cargo.toml" -Recurse

$count = 0
foreach ($file in $cargoFiles) {
    $dir = $file.DirectoryName
    Write-Host "Cleaning project in: $dir" -ForegroundColor Yellow
    Push-Location $dir
    & "C:\Users\dogup\.cargo\bin\cargo.exe" clean
    Pop-Location
    $count++
}

Write-Host "Successfully cleaned $count Rust projects!" -ForegroundColor Green
