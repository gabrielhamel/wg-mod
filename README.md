# WG modding tools

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Features:

- Init mod project
- Mod builder Python only
- Available Windows / Linux / Macos (yes i know)

Comming:

- Auto prepare dev environment
- Mod builder AS3
- WoT debbuger

I will track the development [here](https://github.com/users/gabrielhamel/projects/5)

## Installation

You must have [Cargo / Rust](https://www.rust-lang.org/tools/install) on your machine.

```bash
cargo install wg-mod
```

## Usage

Create a new mod directory

```bash
wg-mod new
```

Build a .wotmod file
```bash
wg-mod build # In mod directory
```

## Development

Run unit-tests in watch mode

```bash
cargo install cargo-watch
cargo watch -x test
```
