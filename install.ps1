# Ensure the script runs with administrator privileges
if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Error "This script must be run as Administrator."
    exit 1
}

# Determine system architecture
$arch = (Get-CimInstance Win32_OperatingSystem).OSArchitecture
switch ($arch) {
    "64-bit" {
        $target = "x86_64-pc-windows-msvc"
    }
    "ARM64" {
        $target = "aarch64-pc-windows-msvc"
    }
    default {
        Write-Error "Unsupported architecture: $arch"
        exit 1
    }
}

# Define download URL and paths
$releaseUrl = "https://github.com/SpikeHD/swerve/releases/latest/download/swerve-$target.exe"
$tempPath = [System.IO.Path]::GetTempFileName() + ".exe"
$installPath = "C:\Program Files\Swerve"

# Download Swerve binary
Write-Host "Downloading Swerve for $arch system..."
Invoke-WebRequest -Uri $releaseUrl -OutFile $tempPath -UseBasicParsing

# Make the install directory if it doesn't exist
if (-not (Test-Path $installPath)) {
    New-Item -ItemType Directory -Path $installPath
}

# Move the binary to the install directory
Move-Item -Path $tempPath -Destination "$installPath\swerve.exe" -Force

# Add install directory to PATH if not already added
$path = [Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::Machine)
if (-not $path.Contains($installPath)) {
    Write-Host "Adding $installPath to system PATH..."
    [Environment]::SetEnvironmentVariable("Path", "$path;$installPath", [System.EnvironmentVariableTarget]::Machine)
}

Write-Host "Swerve installed successfully at $installPath"