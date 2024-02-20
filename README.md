# WFA QT simulator

QT simulator for unit test WFA QT DUT by running individual commands through an interactive shell.

This is written in Rust and uses cargo as the build system.

## Install
Install Rust using `rustup` from [here](https://www.rust-lang.org/tools/install). This will install `rustc` and `cargo` on your system.


## Build (using cargo)
Use the following command to build the project

```shell
    $ cargo build --release
```

## Usage

```shell
$ ./target/release/wfa-qt-sim -h
IP address and port of DUT

Usage: wfa-qt-sim --ip <IP> --port <PORT>

Options:
  -i, --ip <IP>
  -p, --port <PORT>
  -h, --help         Print help
```

## Known issues

None


## References

1. [WFA QT Intro](https://www.wi-fi.org/file/quicktrack-highlights)
2. [WFA QT Source](https://github.com/Wi-FiQuickTrack/Wi-FiQuickTrack-ControlAppC)