# bitsrun

[![GitHub Workflow Status (CI)](https://img.shields.io/github/actions/workflow/status/spencerwooo/bitsrun-rs/ci.yml?logo=github&label=ci&labelColor=%23223227)](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/ci.yml)
[![GitHub Workflow Status (Release)](https://img.shields.io/github/actions/workflow/status/spencerwooo/bitsrun-rs/release.yml?logo=github&label=release&labelColor=%23223227)](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/release.yml)
[![GitHub release](https://img.shields.io/github/v/release/spencerwooo/bitsrun-rs?logo=github&labelColor=%23223227)](https://github.com/spencerwooo/bitsrun-rs/releases/latest)
[![Crates.io](https://img.shields.io/crates/d/bitsrun?logo=rust&labelColor=%23223227&color=%23dec867)](https://crates.io/crates/bitsrun)

🌐 A headless login and logout CLI for 10.0.0.55 at BIT, now in Rust.

![CleanShot 2023-12-04 at 16 47 26@2x](https://github.com/spencerwooo/bitsrun-rs/assets/32114380/23343ba1-961c-41aa-b4b6-c09da93fb699)

## Install

#### One-line install (Linux / macOS, recommended)

- `curl -fsSL https://cdn.jsdelivr.net/gh/spencerwooo/bitsrun-rs@main/install.sh | sh -`

#### Ubuntu / Debian (recommended for `systemd` support)

- Download the latest `.deb` package from [Releases](https://github.com/spencerwooo/bitsrun-rs/releases/latest).
- `sudo apt install </path/to/file>.deb`

**If `bitsrun.service` systemd service required:**

- Edit `/lib/systemd/system/bitsrun.service` to specify absolute config path
- Then start service with `sudo systemctl start bitsrun`

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

To keep the session alive, use `bitsrun keep-alive`:

```console
$ bitsrun keep-alive
 INFO  bitsrun::daemon > starting daemon (<username>) with polling interval=3600s
 INFO  bitsrun::daemon > <ip> (<username>): login success,
 ...
 ^C INFO  bitsrun::daemon > <username>: gracefully exiting
```

> [!NOTE]
> Use available system service managers to run `bitsrun keep-alive` as a daemon. (e.g., `systemd` for Linux, `launchd` for macOS, and Windows Service for Windows).

## Available commands

```console
$ bitsrun --help
A headless login and logout CLI for 10.0.0.55 at BIT

Usage: bitsrun [OPTIONS] [COMMAND]

Commands:
  login         Login to the campus network
  logout        Logout from the campus network
  status        Check device login status
  config-paths  List all possible config file paths
  keep-alive    Poll the server with login requests to keep the session alive
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
  "dm": true,
  "poll_interval": 3600
}
```

- **`dm` is for specifying whether the current device is a dumb terminal, and requires logging out through the alternative endpoint. Set to `true` (no quotes!) if the device you are working with is a dumb terminal.**
- `poll_interval` is an optional field for specifying the interval (in seconds) of polling login requests. Default is `3600` seconds (1 hour). Used by `bitsrun keep-alive` only.

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
