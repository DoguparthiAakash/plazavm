@echo off
echo Finding and cleaning all Rust projects...

setlocal enabledelayedexpansion
set count=0

for /r "%~dp0" %%i in (Cargo.toml) do (
    if exist "%%i" (
        echo Cleaning project in: %%~dpi
        pushd "%%~dpi"
        "C:\Users\dogup\.cargo\bin\cargo.exe" clean
        popd
        set /a count+=1
    )
)

echo Successfully cleaned !count! Rust projects!
pause
