<div align="center">
  <h1>Swerve</h1>
  <p>Speedy, no-fuss local webserver for testing/serving static files or directories.</p>
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

<img width="100%" src="https://github.com/user-attachments/assets/61993ca9-aca0-4e5c-b4d6-c6fbcc98394b" />

# Features

* Multithreaded with a customizable thread pool
* Native ARM support
* Serve directories, webserver directory index style
* Test and serve static sites of all kinds (regular HTML, built React, etc.)
* Serve static sites in Docker
* Optional hot reloading
* Optional basic auth
* Glob patterns for including/excluding files

# Table of Contents
* [Installation](#installation)
  * [Package Repositories](#package-repositories)
  * [Manual Installation](#manual-installation)
* [Usage](#usage)
  * [Usage in Docker](#usage-in-docker)
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

> [!NOTE]
> Maintaining `swerve` somewhere else? Feel free to add it here!

## Manual Installation

You can obtain binaries through [releases](https://github.com/SpikeHD/swerve/releases/), [GitHub Actions](https://github.com/SpikeHD/swerve/actions?query=workflow%3Abuild) artifacts, or by building from source!

### Windows

```powershell
# Run the install script
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/SpikeHD/swerve/refs/heads/main/install.ps1" -OutFile "$env:TEMP\install.ps1"; PowerShell -ExecutionPolicy Bypass -File "$env:TEMP\install.ps1"

# You can uninstall by deleting C:\Program Files\Swerve
del C:\Program Files\Swerve
```

### Linux

```shell
# Run the install script
curl -fsSL https://raw.githubusercontent.com/SpikeHD/swerve/refs/heads/main/install.sh | sudo bash

# You can uninstall by removing the binary from /usr/local/bin
rm /usr/local/bin/swerve
```

### macOS

```shell
# Run the install script
curl -fsSL https://raw.githubusercontent.com/SpikeHD/swerve/refs/heads/main/install.sh | sudo bash

# You can uninstall by removing the binary from /usr/local/bin
rm /usr/local/bin/swerve
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

# Enable hot reloading. This creates a websocket on port 8087.
swerve -H 8087

# Set basic auth credentials
swerve --basic-auth username:password

# Include files using a glob pattern
swerve -i *.html -i *.css -i *.js

# Exclude files using a glob pattern
swerve -e *.txt -e *.md

# Expose to the internet
swerve -p 8080 --bind 0.0.0.0
```

## Usage in Docker

```dockerfile
FROM ubuntu:latest

RUN echo "<html><body><h1>Hello World</h1></body></html>" > ./index.html

RUN apt update && apt install -y curl
RUN curl -fsSL https://raw.githubusercontent.com/SpikeHD/swerve/refs/heads/main/install.sh | bash

EXPOSE 8080

CMD ["swerve", "--port", "8080", "--bind", "0.0.0.0", "-r"]
```

You can access this by running the following:
```sh
docker build . --tag swerve-test
docker run -p 8080:8080 swerve-test
```

# Building

## Prerequisites

* [Rust](https://www.rust-lang.org/tools/install)

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
* [x] Optional hot-reloading
* [x] Multithreading
* [x] More details in directory listings (modified date, size, etc.)
* [x] Basic auth

# Contributions

Issues, PRs, etc. are all welcome!
