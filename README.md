<div align="center">
  <h1>Swerve</h1>
  <p>Speedy, no-fuss serving of directories on localhost.</p>
</div>

<div align="center">
  <img src="https://img.shields.io/github/actions/workflow/status/SpikeHD/swerve/build.yml" />
  <img src="https://img.shields.io/github/repo-size/SpikeHD/swerve" />
  <img src="https://img.shields.io/github/commit-activity/m/SpikeHD/swerve" />
</div>

<div align="center">
  <img src="https://img.shields.io/github/release-date/SpikeHD/swerve" />
  <img src="https://img.shields.io/github/stars/SpikeHD/swerve" />
</div>

<details>
  <summary>Video demo</summary>

  https://github.com/SpikeHD/swerve/assets/25207995/07ccc8b5-797f-4675-b266-6a0e8fbcec8b
</details>

# Table of Contents
* [Installation](#installation)
  * [Package Repositories](#package-repositories)
  * [Manual Installation](#manual-installation)
* [Usage](#usage)
* [Building](#building)
  * [Prerequisites](#prerequisites)
  * [Steps](#steps)
* [TODO](#todo)
* [Contributions](#contributions)

# Installation

## Package Repositories

### Windows

* WinGet
  ```sh
  winget install SpikeHD.swerve
  ```

### Linux

Nowhere... yet

### MacOS

Nowhere... yet

> [!NOTE]
> Maintaining `swerve` somewhere else? Feel free to add it here!

## Manual Installation

You can obtain binaries through [releases](https://github.com/SpikeHD/swerve/releases/), [GitHub Actions](https://github.com/SpikeHD/swerve/actions?query=workflow%3Abuild) artifacts, or by building from source!

### Windows

```shell
# x86/64
iwr https://github.com/SpikeHD/swerve/releases/latest/download/swerve-x86_64-pc-windows-msvc.exe -OutFile swerve.exe

# ARM64
iwr https://github.com/SpikeHD/swerve/releases/latest/download/swerve-aarch64-pc-windows-msvc.exe  -OutFile swerve.exe
```

Then you can move it somewhere and add it to your PATH variable.

### Linux

```shell
# x86/64
wget https://github.com/SpikeHD/swerve/releases/latest/download/swerve-x86_64-unknown-linux-gnu -O swerve

# ARM64
wget https://github.com/SpikeHD/swerve/releases/latest/download/swerve-aarch64-unknown-linux-gnu -O swerve

# ARM v7
wget https://github.com/SpikeHD/swerve/releases/latest/download/swerve-armv7-unknown-linux-gnueabihf -O swerve

# Move to bin folder
sudo mv swerve /usr/local/bin
```

### macOS

```shell
# x86/64
curl -L https://github.com/SpikeHD/swerve/releases/latest/download/swerve-x86_64-apple-darwin -o swerve

# ARM64
curl -L https://github.com/SpikeHD/swerve/releases/latest/download/swerve-aarch64-apple-darwin -o swerve

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

# Include files using a glob pattern
swerve -i *.html -i *.css -i *.js

# Exclude files using a glob pattern
swerve -e *.txt -e *.md
```

# Building

## Prerequisites

* [Rust](https://www.rust-lang.org/tools/install)
* A computer

## Steps

1. Clone the repository
  ```shell
  git clone https://github.com/SpikeHD/swerve.git
  ```
2. `cd` into the repository
  ```shell
  cd swerve
  ```
3. Build the project
  ```shell
  cargo build --release
  ```

The binary will be in `target/release/`.

# TODO

* [x] Include/exclude files/folders/globs
* [x] Embedded HTML/CSS for displaying directories

# Contributions

Issues, PRs, etc. are all welcome!
