# catsh
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/jheysonsaav/catsh/Ci?label=Ci&logo=github)
![GitHub release](https://img.shields.io/github/v/release/jheysonsaav/catsh?include_prereleases&label=Release)
![GitHub license](https://img.shields.io/github/license/jheysonsaav/catsh)
![GitHub contributors](https://img.shields.io/github/contributors/jheysonsaav/catsh?label=Contributors)

A catsh cross-platform shell

## Usage
For create a new catsh session run the command `catsh start`

## Requirements
- [Rust](https://www.rust-lang.org/)

## Compiling
1. keep the [requirements](#Requirements) in mind
2. clone this repository:
  - **Github Cli**: `gh repo clone jheysonsaav/catsh`
  - **Git**: `git clone https://github.com/jheysonsaav/catsh.git`
3. go to the folder called `catsh` and run the `cargo build --release` command
4. you have succeeded, now you can run catsh with the `target/release/catsh` command for unix systems or `.\target\release\catsh.exe` for windows
