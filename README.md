service-manager-ctl
===================

[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](https://opensource.org/licenses/Apache-2.0)

CLI tool to install & manage: Windows, OpenRC, systemd, rc.d, and launchd services.

FYI: This wraps https://github.com/chipsenkbeil/service-manager-rs; plan to send a PR to close my https://github.com/chipsenkbeil/service-manager-rs/issues/20

## `--help` | `help`

    CLI tool to install & manage: Windows, OpenRC, systemd, rc.d, and launchd services
    
    Usage: service-manager-ctl [OPTIONS] [COMMAND]
    
    Commands:
      install  deploys and instantiated the service
      status   Get the service status info
      start    Start the service
      stop     Stop the service
      help     Print this message or the help of the given subcommand(s)
    
    Options:
      -d, --debug...
              Turn debugging information on
    
          --service-manager <SERVICE_MANAGER>
              Possible values:
              - launchd: Use launchd to manage the service
              - openrc:  Use OpenRC to manage the service
              - rcd:     Use rc.d to manage the service
              - sc:      Use Windows service controller to manage the service
              - systemd: Use systemd to manage the service
              - winsw:   Use WinSW to manage the service
    
      -h, --help
              Print help (see a summary with '-h')
    
      -V, --version
              Print version

## `--version`

    service-manager-ctl 0.0.1

## `install`

    deploys and instantiated the service
    
    Usage: service-manager-ctl install [OPTIONS] --program <PROGRAM> <LABEL>
    
    Arguments:
      <LABEL>
              Label associated with the service
              
              E.g. `org.example.my_application`
    
    Options:
      -p, --program <PROGRAM>
              Path to the program to run
              
              E.g. `/usr/local/bin/my-program`
    
          --args <ARGS>
              Arguments to use for the program
              
              E.g. `--arg`, `value`, `--another-arg`
    
      -c, --contents <CONTENTS>
              Optional contents of the service file for a given ServiceManager to use instead of the default template
    
      -u, --username <USERNAME>
              Optionally supply the user the service will run as
              
              If not specified, the service will run as the root or Administrator user.
    
      -w, --working-directory <WORKING_DIRECTORY>
              Optionally specify a working directory for the process launched by the service
    
      -e, --environment <ENVIRONMENT>
              Optionally specify a list of environment variables to be passed to the process launched by the service
    
      -a, --autostart
              Specify whether the service should automatically start on reboot
    
          --disable-restart-on-failure
              Optionally disable a service from restarting when it exits with a failure
              
              This could overwrite the platform specific service manager config.
    
      -h, --help
              Print help (see a summary with '-h')

## `status`

    Get the service status info
    
    Usage: service-manager-ctl status <LABEL>
    
    Arguments:
      <LABEL>
              Label associated with the service
              
              E.g. `org.example.my_application`
    
    Options:
      -h, --help
              Print help (see a summary with '-h')

## `start`

    Start the service
    
    Usage: service-manager-ctl start <LABEL>
    
    Arguments:
      <LABEL>
              Label associated with the service
              
              E.g. `org.example.my_application`
    
    Options:
      -h, --help
              Print help (see a summary with '-h')

## `stop`

    Stop the service
    
    Usage: service-manager-ctl stop <LABEL>
    
    Arguments:
      <LABEL>
              Label associated with the service
              
              E.g. `org.example.my_application`
    
    Options:
      -h, --help
              Print help (see a summary with '-h')

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
