# bitsrun

[![CI](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/ci.yml)
[![Release](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/release.yml/badge.svg)](https://github.com/spencerwooo/bitsrun-rs/actions/workflows/release.yml)
![GitHub release (with filter)](https://img.shields.io/github/v/release/spencerwooo/bitsrun-rs)

🌐 A headless login and logout CLI app for 10.0.0.55 at BIT, now in Rust.

![screenshot](https://github.com/spencerwooo/bitsrun-rs/assets/32114380/011e7591-1474-4df8-a371-7a9da7629959)

## Usage

```console
$ bitsrun help
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

## License

[MIT](./LICENSE)
