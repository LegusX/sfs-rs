# sfs-rs

## Description

sfs-rs is a tool to help manage the priority of library selection when using Steam Family Sharing. sfs-rs is heavily inspired by [SFS-SELECT](https://steamcommunity.com/groups/familysharing/discussions/1/3068621701744549116/) and seeks to accomplish the same goals, but without making your antivirus freak out.

## Installation

### Windows

Download the executable from the [releases](https://github.com/LegusX/sfs-rs/releases/latest) tab and run it.

### Linux

There are two possible methods for running on linux:

#### Pre-built executable

This is the simplest way, but may not work, depending on your distribution.

1. Download the latest release from the [releases](https://github.com/LegusX/sfs-rs/releases/latest) tab.
2. Mark as executable `chmod +x sfs-rs` and run.

#### Building from scratch

If the executable doesn't work for one reason or another, you can try building from scratch:

1. Make sure you have rust and cargo installed.
2. [Get a Steam API key](https://steamcommunity.com/dev/apikey)
3. Clone the repository `git clone git@github.com:LegusX/sfs-rs.git`
4. cd into the repository and run `STEAM_API="STEAM_KEY_GOES_HERE" cargo install --path .`
5. If you've already added `~/.cargo/bin` to your path you should be able to just run `sfs-rs`, if not, you can either add it to your path, or just run `~/.cargo/bin/sfs-rs`

## Usage

Drag the users into the order that you want their libraries to be used in. For instance, if Friend A and Friend B both own Skyrim, and Friend A is online and using their library, you'll want to drag Friend B to the top so that you borrow Skyrim from their library instead of Friend A's library
