$ErrorActionPreference = "Stop"

$ProjectRoot = (Resolve-Path "$PSScriptRoot\..\..").Path
$TargetDir = "$ProjectRoot\target\release"
$StagingDir = "$ProjectRoot\scripts\windows\staging"
$OutDir = "$ProjectRoot\dist"

Write-Host "Building PlazaVM Installer..."

# Step 1: Build PlazaVM
Write-Host "Compiling PlazaVM Release..."
Set-Location $ProjectRoot
cargo build --release

# Step 2: Prepare Staging Directory
Write-Host "Preparing Staging Directory..."
if (Test-Path $StagingDir) { Remove-Item -Recurse -Force $StagingDir }
New-Item -ItemType Directory -Path $StagingDir | Out-Null
if (!(Test-Path $OutDir)) { New-Item -ItemType Directory -Path $OutDir | Out-Null }

Copy-Item "$TargetDir\plaza-cli.exe" "$StagingDir\plaza.exe"

# Create Uninstaller Script
$UninstallerScript = @"
`$ErrorActionPreference = "Stop"
Write-Host "======================================"
Write-Host " PlazaVM Uninstaller "
Write-Host "======================================"

`$InstallDir = "`$env:LOCALAPPDATA\PlazaVM"
`$BinDir = "`$InstallDir\bin"

# Kill processes
Get-Process -Name "plaza" -ErrorAction SilentlyContinue | Stop-Process -Force

# Remove from PATH
`$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
`$PathParts = `$UserPath -split ";"
`$NewParts = @()
foreach (`$part in `$PathParts) {
    if (`$part -and (`$part.Trim() -ne `$BinDir)) {
        `$NewParts += `$part
    }
}
`$NewPath = `$NewParts -join ";"
if (`$NewPath -ne `$UserPath) {
    Write-Host "Removing PlazaVM from user PATH..."
    [Environment]::SetEnvironmentVariable("PATH", `$NewPath, "User")
}

# Remove directory (except itself if running from it, though usually we just force it)
if (Test-Path `$InstallDir) {
    Write-Host "Deleting installation directory..."
    Remove-Item -Recurse -Force `$InstallDir -ErrorAction SilentlyContinue
}
Write-Host "PlazaVM uninstalled successfully."
Start-Sleep -Seconds 3
"@
Set-Content "$StagingDir\uninstall.ps1" $UninstallerScript

# Step 3: Zip staging directory
Write-Host "Compressing binaries..."
$ZipPath = "$OutDir\PlazaVM_Payload.zip"
if (Test-Path $ZipPath) { Remove-Item -Force $ZipPath }
Compress-Archive -Path "$StagingDir\*" -DestinationPath $ZipPath

# Step 4: Convert zip to base64
$Bytes = [System.IO.File]::ReadAllBytes($ZipPath)
$Base64 = [System.Convert]::ToBase64String($Bytes)

# Step 5: Create self-extracting script
Write-Host "Creating self-extracting installer..."
$InstallerPath = "$OutDir\PlazaVM_Setup.ps1"

$SetupScript = @"
`$ErrorActionPreference = "Stop"

Write-Host "======================================"
Write-Host " PlazaVM Installation Setup "
Write-Host "======================================"

`$InstallDir = "`$env:LOCALAPPDATA\PlazaVM"
`$BinDir = "`$InstallDir\bin"
`$WorkspacesDir = "`$InstallDir\workspaces"
`$ImagesDir = "`$InstallDir\images"

Write-Host "Installing PlazaVM to `$InstallDir..."

# Kill existing instances to prevent file lock
Get-Process -Name "plaza" -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Seconds 1

# Clean up existing binary to prevent Expand-Archive access denied
if (Test-Path `$BinDir) {
    Remove-Item -Recurse -Force `$BinDir -ErrorAction SilentlyContinue
}

# Create directories
New-Item -ItemType Directory -Force -Path `$BinDir | Out-Null
New-Item -ItemType Directory -Force -Path `$WorkspacesDir | Out-Null
New-Item -ItemType Directory -Force -Path `$ImagesDir | Out-Null

# Extract payload
`$ZipBase64 = "$Base64"
`$ZipBytes = [System.Convert]::FromBase64String(`$ZipBase64)
`$TempZip = [System.IO.Path]::GetTempFileName() + ".zip"
[System.IO.File]::WriteAllBytes(`$TempZip, `$ZipBytes)

Write-Host "Extracting binaries..."
Expand-Archive -Path `$TempZip -DestinationPath `$BinDir -Force
Remove-Item -Force `$TempZip

# Setup PATH (Prepend)
`$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
`$PathParts = `$UserPath -split ";"
`$NewParts = @(`$BinDir)
foreach (`$part in `$PathParts) {
    if (`$part -and (`$part.Trim() -ne `$BinDir)) {
        `$NewParts += `$part
    }
}
`$NewPath = `$NewParts -join ";"
if (`$NewPath -ne `$UserPath) {
    Write-Host "Adding PlazaVM to user PATH..."
    [Environment]::SetEnvironmentVariable("PATH", `$NewPath, "User")
    `$env:PATH = `$NewPath
}

# Download QEMU note
Write-Host "`nChecking QEMU dependencies..."
`$QemuCmd = Get-Command "qemu-system-x86_64.exe" -ErrorAction SilentlyContinue
if (-not `$QemuCmd) {
    Write-Host "QEMU not found in PATH."
    Write-Host "NOTE: For full validation, please ensure QEMU is installed via MSYS2 or official installer."
    Write-Host "      Download: https://qemu.weilnetz.de/w64/"
}

Write-Host "`nPlazaVM installation complete! Open a new terminal and run 'plaza doctor'."
Start-Sleep -Seconds 3
"@

Set-Content $InstallerPath $SetupScript
Write-Host "PS1 Installer created at: $InstallerPath"

# Step 6: Generate C# Wrapper for .EXE
Write-Host "Creating C# wrapper for native .exe installer..."
$CsPath = "$OutDir\Installer.cs"
$ExePath = "$OutDir\PlazaVM_Setup.exe"

$CsScript = @"
using System;
using System.IO;
using System.IO.Compression;
using System.Reflection;
using System.Windows.Forms;
using System.Diagnostics;

namespace PlazaVMInstaller
{
    class Program
    {
        static void Main(string[] args)
        {
            try
            {
                string appData = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
                string installDir = Path.Combine(appData, `"PlazaVM`");
                string binDir = Path.Combine(installDir, `"bin`");

                // Read resource
                Assembly assembly = Assembly.GetExecutingAssembly();
                using (Stream resStream = assembly.GetManifestResourceStream(`"PlazaVM_Payload.zip`"))
                {
                    if (resStream == null)
                    {
                        MessageBox.Show(`"Installer payload missing from executable!`", `"PlazaVM Installer Error`", MessageBoxButtons.OK, MessageBoxIcon.Error);
                        return;
                    }

                    try
                    {
                        foreach (var process in Process.GetProcessesByName(`"plaza`"))
                        {
                            process.Kill();
                            process.WaitForExit(2000);
                        }
                    }
                    catch { }

                    if (Directory.Exists(installDir))
                    {
                        try {
                            Directory.Delete(installDir, true);
                        } catch {
                            // If delete fails, just try to delete the bin folder contents
                            try { Directory.Delete(binDir, true); } catch { }
                        }
                    }
                    Directory.CreateDirectory(installDir);
                    Directory.CreateDirectory(binDir);

                    using (ZipArchive archive = new ZipArchive(resStream))
                    {
                        archive.ExtractToDirectory(binDir);
                    }
                }

                // Update PATH (Prepend so it takes precedence over old Python installs)
                string userPath = Environment.GetEnvironmentVariable(`"PATH`", EnvironmentVariableTarget.User) ?? `"`";
                string[] pathParts = userPath.Split(new char[] { ';' }, StringSplitOptions.RemoveEmptyEntries);
                var newParts = new System.Collections.Generic.List<string>();
                newParts.Add(binDir); // Prepend it
                foreach(var part in pathParts) {
                    if (!string.Equals(part.Trim(), binDir.Trim(), StringComparison.OrdinalIgnoreCase)) {
                        newParts.Add(part);
                    }
                }
                string newPath = string.Join(`";`", newParts);
                if (newPath != userPath) {
                    Environment.SetEnvironmentVariable(`"PATH`", newPath, EnvironmentVariableTarget.User);
                }

                MessageBox.Show(`"PlazaVM has been installed successfully!\n\nOpen a new terminal and type 'plaza --help' to get started.`", 
                    `"PlazaVM Installer`", MessageBoxButtons.OK, MessageBoxIcon.Information);
            }
            catch (Exception ex)
            {
                MessageBox.Show(`"Installation failed:\n`" + ex.Message, `"PlazaVM Installer Error`", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }
    }
}
"@

Set-Content $CsPath $CsScript

# Find csc.exe
$CscPath = "C:\Windows\Microsoft.NET\Framework64\v4.0.30319\csc.exe"
if (!(Test-Path $CscPath)) {
    Write-Warning "csc.exe not found at $CscPath. Cannot build .exe"
} else {
    Write-Host "Compiling .exe..."
    $IconFlag = ""
    if (Test-Path "$ProjectRoot\logo.ico") {
        $IconFlag = "/win32icon:`"$ProjectRoot\logo.ico`""
    }
    
    $CscArgs = "/nologo /target:winexe /reference:System.IO.Compression.dll /reference:System.IO.Compression.FileSystem.dll /resource:`"$ZipPath`",PlazaVM_Payload.zip $IconFlag /out:`"$ExePath`" `"$CsPath`""
    Invoke-Expression "& `"$CscPath`" $CscArgs"
    if ($LASTEXITCODE -eq 0) {
        Write-Host "EXE Installer created at: $ExePath"
    } else {
        Write-Warning "Failed to compile .exe"
    }
}

Remove-Item -Force $ZipPath
Remove-Item -Force $CsPath
