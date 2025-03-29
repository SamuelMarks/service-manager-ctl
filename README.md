service-manager-ctl
===================

[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](https://opensource.org/licenses/Apache-2.0)

CLI tool to install & manage: Windows, OpenRC, systemd, rc.d, and launchd services.

FYI: This wraps https://github.com/chipsenkbeil/service-manager-rs; plan to send a PR to close my https://github.com/chipsenkbeil/service-manager-rs/issues/20

## `--help`

    Usage: service-manager-ctl [OPTIONS] [COMMAND]
    
    Commands:
    install  deploys and instantiated the service
    help     Print this message or the help of the given subcommand(s)
    
    Options:
    -d, --debug...  Turn debugging information on
    -h, --help      Print help
    -V, --version   Print version

<hr>

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
