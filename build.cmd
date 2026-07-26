@echo off
where cargo >nul 2>nul
if errorlevel 1 (
    echo Rust is not installed. Please install Rust from https://rustup.rs/
    exit /b 1
)

cargo build --release
if errorlevel 1 (
    echo Build failed.
    exit /b 1
)

copy target\release\CFGBeast.exe %~dp0
CertUtil -hashfile "%~dp0CFGBeast.exe" SHA256 > "%~dp0CFGBeast.exe.sha256.txt"

echo Build complete.
type "%~dp0CFGBeast.exe.sha256.txt"