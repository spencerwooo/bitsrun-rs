# bitsrun

[![CI](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/ci.yml)
[![Release](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/release.yml/badge.svg)](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/release.yml)
[![GitHub release (with filter)](https://img.shields.io/github/v/release/spencerwooo/bitsrun-rs)](https://github.com/spencerwooo/bitsrun-rs/releases/latest)

🌐 A headless login and logout CLI app for 10.0.0.55 at BIT, now in Rust.

![screenshot](https://github.com/spencerwooo/bitsrun-rs/assets/32114380/011e7591-1474-4df8-a371-7a9da7629959)

## Installation

Download the latest binary from [GitHub Releases](https://github.com/spencerwooo/bitsrun-rs/releases/latest).

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

## Credentials

User credentials are stored in config file `bit-user.json`. Available config file paths can be listed with:

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

## Related

- [`zu1k/srun`](https://github.com/zu1k/srun) - Srun authentication system login tools. (Rust)
- [`Mmx233/BitSrunLoginGo`](https://github.com/Mmx233/BitSrunLoginGo) - 深澜校园网登录脚本 Go 语言版 (Go)
- [`vouv/srun`](https://github.com/vouv/srun) - An efficient client for BIT campus network. (Go)
- [`BITNP/bitsrun`](https://github.com/BITNP/bitsrun) - A headless login / logout script for 10.0.0.55 at BIT. (Python)

## License

[MIT](./LICENSE)
