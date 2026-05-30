@echo off
echo Setting up MSVC environment...
call "D:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat" x64

echo Adding Rust to PATH...
set PATH=C:\Users\admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin;C:\Users\admin\.cargo\bin;%PATH%

echo Starting build...
call npm run tauri -- build --no-bundle --target x86_64-pc-windows-msvc
echo BUILD_EXIT_CODE=%ERRORLEVEL%
pause
