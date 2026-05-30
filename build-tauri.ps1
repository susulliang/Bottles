# build-tauri.ps1
# Configure MSVC environment
$vsInstallPath = "D:\Program Files\Microsoft Visual Studio\2022\Community"
$msvcVersion = "14.44.35207"
$msvcPath = "$vsInstallPath\VC\Tools\MSVC\$msvcVersion"
$windowsKitPath = "C:\Program Files (x86)\Windows Kits\10"
$windowsSdkVersion = "10.0.26100.0"

# Set PATH
$env:PATH = @(
    "$msvcPath\bin\Hostx64\x64",
    "$windowsKitPath\bin\$windowsSdkVersion\x64",
    "$windowsKitPath\bin\x64",
    "C:\Users\admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin",
    "C:\Users\admin\.cargo\bin",
    "C:\Program Files\nodejs",
    "C:\Users\admin\.cargo\bin",
    $env:PATH
) -join ';'

# Set INCLUDE - MSVC
$env:INCLUDE = @(
    "$msvcPath\include",
    # Windows SDK
    "$windowsKitPath\include\$windowsSdkVersion\ucrt",
    "$windowsKitPath\include\$windowsSdkVersion\um",
    "$windowsKitPath\include\$windowsSdkVersion\shared",
    "$windowsKitPath\include\$windowsSdkVersion\winrt",
    # VC tools
    "$vsInstallPath\VC\Tools\MSVC\$msvcVersion\include",
    "$vsInstallPath\VC\Tools\MSVC\$msvcVersion\atlmfc\include",
    "$vsInstallPath\VC\Auxiliary\VS\include"
) -join ';'

# Set LIB
$env:LIB = @(
    "$msvcPath\lib\x64",
    "$windowsKitPath\Lib\$windowsSdkVersion\ucrt\x64",
    "$windowsKitPath\Lib\$windowsSdkVersion\um\x64"
) -join ';'

# Set LIBPATH
$env:LIBPATH = @(
    "$msvcPath\lib\x64",
    "$windowsKitPath\Lib\$windowsSdkVersion\ucrt\x64",
    "$windowsKitPath\Lib\$windowsSdkVersion\um\x64"
) -join ';'

# Set VS environment variables
$env:VSINSTALLDIR = "$vsInstallPath\"
$env:VCINSTALLDIR = "$vsInstallPath\VC\"
$env:WindowsSdkDir = "$windowsKitPath\"
$env:WindowsSDKVersion = "$windowsSdkVersion"

# Clean target first
$targetDir = Join-Path $PSScriptRoot "src-tauri\target"
if (Test-Path $targetDir) {
    Remove-Item -Recurse -Force $targetDir -ErrorAction SilentlyContinue
    Write-Host "Cleaned target directory"
}

# Set environment variables
$env:RUSTFLAGS = ""
$env:CC = ""
$env:CXX = ""

Write-Host "MSVC Environment configured"
Write-Host "VSINSTALLDIR=$env:VSINSTALLDIR"
Write-Host "CC=$env:CC"
Write-Host "CXX=$env:CXX"
Write-Host ""

# Build
Write-Host "Building Tauri app..."
& "C:\Program Files\nodejs\npm.cmd" run tauri -- build --no-bundle --target x86_64-pc-windows-msvc
Write-Host "BUILD_EXIT_CODE=$LASTEXITCODE"
