<div align="center">
  <h1>Swerve</h1>

  <p>No-fuss serving of directories to localhost</p>
</div>

# Table of Contents
* [Installation](#installation)
  * [Windows](#windows)
  * [Linux](#linux)
  * [macOS](#macos)
* [Usage](#usage)
* [Contributions](#contributions)

# Installation

## Package Repositories

Not yet available in package repositories. If you'd like to add it to your favorite one, feel free!

## Manual Installation

### Windows

```shell
# x86/64
iwr https://github.com/SpikeHD/swerve/releases/download/latest/swerve_win_x64.exe -OutFile swerve.exe

# ARM64
iwr https://github.com/SpikeHD/swerve/releases/download/latest/swerve_win_arm64.exe -OutFile swerve.exe
```

Then you can move it somewhere and add it to your PATH variable.

### Linux

```shell
# x86/64
wget https://github.com/SpikeHD/swerve/releases/download/latest/swerve_linux_x64 -O swerve

# ARM64
wget https://github.com/SpikeHD/swerve/releases/download/latest/swerve_linux_arm64 -O swerve

# Move to bin folder
sudo mv swerve /usr/local/bin
```

### macOS

```shell
# x86/64
curl -L https://github.com/SpikeHD/swerve/releases/download/latest/swerve_macos_x64 -o swerve

# ARM64
curl -L https://github.com/SpikeHD/swerve/releases/download/latest/swerve_macos_arm64 -o swerve

# Move to bin folder
sudo mv swerve /usr/local/bin
```

# Usage

```shell
# Show help
swerve -h

# Serve the current directory
swerve

# Serve a specific directory
swerve path/to/directory

# Specify port
swerve -p 8080
```

# Contributions

Issues, PRs, etc. are all welcome!