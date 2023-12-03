# bitsrun

[![CI](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/ci.yml)
[![Release](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/release.yml/badge.svg)](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/release.yml)
[![GitHub release (with filter)](https://img.shields.io/github/v/release/spencerwooo/bitsrun-rs)](https://github.com/spencerwooo/bitsrun-rs/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/bitsrun?color=rgb(221%2C%20170%2C%2071))](https://crates.io/crates/bitsrun)

🌐 A headless login and logout CLI app for 10.0.0.55 at BIT, now in Rust.

![screenshot](https://github.com/spencerwooo/bitsrun-rs/assets/32114380/011e7591-1474-4df8-a371-7a9da7629959)

## Install

#### One-line install (Linux / macOS, recommended)

- `curl -fsSL https://cdn.jsdelivr.net/gh/spencerwooo/bitsrun-rs@main/install.sh | sh -`

#### Ubuntu / Debian

- Download the latest `.deb` package from [Releases](https://github.com/spencerwooo/bitsrun-rs/releases/latest).
- `sudo dpkg -i <file>.deb`

#### Cargo

- `cargo install bitsrun`

#### Download binary

- Download the latest binary from [Releases](https://github.com/spencerwooo/bitsrun-rs/releases/latest).
- Uncompress file: `tar -xvf <file>.tar.gz`
- Move binary to `$PATH`, such as: `mv <file>/bitsrun ~/.local/bin/`

## Usage

To log into or out of the campus network, simply:

```console
$ bitsrun login -u <username> -p <password>
bitsrun: <ip> (<username>) logged in

$ bitsrun logout -u <username>
bitsrun: <ip> logged out
```

To check device login status:

```console
$ bitsrun status
bitsrun: <ip> (<username>) is online
┌────────────────┬───────────────┬───────────────┬─────────┐
│ Traffic Used   │ Online Time   │ User Balance  │ Wallet  │
├────────────────┼───────────────┼───────────────┼─────────┤
│ 188.10 GiB     │ 2 months      │ 10.00         │ 0.00    │
└────────────────┴───────────────┴───────────────┴─────────┘
```

## Available commands

```console
$ bitsrun --help
A headless login and logout CLI app for 10.0.0.55 at BIT

Usage: bitsrun [OPTIONS] [COMMAND]

Commands:
  login         Login to the campus network
  logout        Logout from the campus network
  status        Check device login status
  config-paths  List all possible config file paths
  help          Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Verbose output
  -h, --help     Print help
  -V, --version  Print version
```

> [!TIP]
> Use environment variable `NO_COLOR=true` to disable colored output.

## Config and credentials

To save your credentials and configurations, create config file `bit-user.json` under an available config path as:

```json
{
  "username": "<username>",
  "password": "<password>",
  "dm": true
}
```

**`dm` is for specifying whether the current device is a dumb terminal, and requires logging out through the alternative endpoint. Set to `true` (no quotes!) if the device you are working with is a dumb terminal.**

Available config file paths can be listed with:

```console
$ bitsrun config-paths
bitsrun: list of possible config paths
┌──────────┬─────────────────────────────────────────────────────────────┐
│ Priority │ Possible Config Path                                        │
├──────────┼─────────────────────────────────────────────────────────────┤
│ 1        │ /Users/spencerwoo/.config/bit-user.json                     │
│ 2        │ /Users/spencerwoo/.config/bitsrun/bit-user.json             │
│ 3        │ /Users/spencerwoo/Library/Preferences/bitsrun/bit-user.json │
│ 4        │ bit-user.json                                               │
└──────────┴─────────────────────────────────────────────────────────────┘
```

> [!NOTE]
> The config file location is OS-dependent. Run the command to check the accepted locations on your system.

**Set permissions of this file to `600` on Linux and macOS, or `bitsrun` will refuse to read it.**

```console
$ chmod 600 <path/to/bit-user.json>
```

## Related

- [`zu1k/srun`](https://github.com/zu1k/srun) - Srun authentication system login tools. (Rust)
- [`Mmx233/BitSrunLoginGo`](https://github.com/Mmx233/BitSrunLoginGo) - 深澜校园网登录脚本 Go 语言版 (Go)
- [`vouv/srun`](https://github.com/vouv/srun) - An efficient client for BIT campus network. (Go)
- [`BITNP/bitsrun`](https://github.com/BITNP/bitsrun) - A headless login / logout script for 10.0.0.55 at BIT. (Python)

## License

[MIT](./LICENSE)
