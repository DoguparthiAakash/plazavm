$ErrorActionPreference = "Stop"

$Root = "e:\plazavm"
$FoundationSrc = "$Root\shared\plaza-foundation\src"

# Create modules
New-Item -ItemType Directory -Path "$FoundationSrc\core" -Force | Out-Null
New-Item -ItemType Directory -Path "$FoundationSrc\events" -Force | Out-Null
New-Item -ItemType Directory -Path "$FoundationSrc\config" -Force | Out-Null
New-Item -ItemType Directory -Path "$FoundationSrc\platform" -Force | Out-Null

# Move code (excluding lib.rs)
Copy-Item -Path "$Root\plaza-core\src\*" -Destination "$FoundationSrc\core\" -Recurse -Force
Remove-Item -Path "$FoundationSrc\core\lib.rs" -Force

Copy-Item -Path "$Root\plaza-events\src\*" -Destination "$FoundationSrc\events\" -Recurse -Force
Remove-Item -Path "$FoundationSrc\events\lib.rs" -Force

Copy-Item -Path "$Root\plaza-config\src\*" -Destination "$FoundationSrc\config\" -Recurse -Force
Remove-Item -Path "$FoundationSrc\config\lib.rs" -Force

Copy-Item -Path "$Root\plaza-platform\src\*" -Destination "$FoundationSrc\platform\" -Recurse -Force
Remove-Item -Path "$FoundationSrc\platform\lib.rs" -Force

# Rewrite plaza-foundation/src/lib.rs
$LibRs = @"
//! # plaza-foundation
//!
//! Absolute bottom-layer dependencies for the PlazaVM ecosystem.

pub mod core;
pub mod events;
pub mod config;
pub mod platform;

// We preserve the existing engine module temporarily if needed by other crates,
// but it should probably move to plaza-workspace.
pub mod engine;
pub mod protocol;
pub mod registry;
"@
Set-Content -Path "$FoundationSrc\lib.rs" -Value $LibRs

Write-Host "Consolidated code into shared/plaza-foundation"
