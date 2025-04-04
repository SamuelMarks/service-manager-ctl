use std::ffi::OsString;
use std::path::PathBuf;

use clap::Parser;

/// Parse a single key-value pair
fn parse_key_val<T, U>(
    s: &str,
) -> Result<(T, U), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[arg(long)]
    service_manager: Option<service_manager::ServiceManagerKind>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// deploys and instantiated the service
    Install {
        /// Label associated with the service
        ///
        /// E.g. `org.example.my_application`
        label: String,

        /// Path to the program to run
        ///
        /// E.g. `/usr/local/bin/my-program`
        #[arg(short, long)]
        program: PathBuf,

        /// Arguments to use for the program
        ///
        /// E.g. `--arg`, `value`, `--another-arg`
        #[arg(long)]
        args: Vec<OsString>,

        /// Optional contents of the service file for a given ServiceManager
        /// to use instead of the default template.
        #[arg(short, long)]
        contents: Option<String>,

        /// Optionally supply the user the service will run as
        ///
        /// If not specified, the service will run as the root or Administrator user.
        #[arg(short, long)]
        username: Option<String>,

        /// Optionally specify a working directory for the process launched by the service
        #[arg(short, long)]
        working_directory: Option<PathBuf>,

        /// Optionally specify a list of environment variables to be passed to the process launched by
        /// the service
        #[arg(short, long, value_parser = parse_key_val::<String, String>)]
        environment: Option<Vec<(String, String)>>,

        /// Specify whether the service should automatically start on reboot
        #[arg(short, long)]
        autostart: bool,

        /// Optionally disable a service from restarting when it exits with a failure
        ///
        /// This could overwrite the platform specific service manager config.
        #[arg(long)]
        disable_restart_on_failure: bool,
    },

    /// Get the service status info
    Status {
        /// Label associated with the service
        ///
        /// E.g. `org.example.my_application`
        label: String,
    },

    /// Start the service
    Start {
        /// Label associated with the service
        ///
        /// E.g. `org.example.my_application`
        label: String,
    },

    /// Stop the service
    Stop {
        /// Label associated with the service
        ///
        /// E.g. `org.example.my_application`
        label: String,
    },
}

fn main() -> std::process::ExitCode {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    let manager = match cli.service_manager {
        None =>
        // Get generic service by detecting what is available on the platform
        {
            <dyn service_manager::ServiceManager>::native()
                .expect("Failed to detect management platform")
        }
        Some(kind) => Box::new(service_manager::TypedServiceManager::target(kind)),
    };
    match &cli.command {
        Some(Commands::Install {
            label,
            program,
            args,
            contents,
            username,
            working_directory,
            environment,
            autostart,
            disable_restart_on_failure,
        }) => {
            // Create a label for our service
            let label: service_manager::ServiceLabel = label.parse().unwrap();

            // Install our service using the underlying service management platform
            manager
                .install(service_manager::ServiceInstallCtx {
                    label,
                    program: program.to_owned(),
                    args: args.to_owned(),
                    contents: contents.to_owned(),
                    username: username.to_owned(),
                    working_directory: working_directory.to_owned(),
                    environment: environment.to_owned(),
                    autostart: autostart.to_owned(),
                    disable_restart_on_failure: disable_restart_on_failure.to_owned(),
                })
                .unwrap()
        }
        None => {}
        Some(Commands::Status { label }) => {
            match manager
                .status(service_manager::ServiceStatusCtx {
                    label: label.parse().unwrap(),
                })
                .unwrap()
            {
                service_manager::ServiceStatus::NotInstalled => {
                    println!("{{\"label\": \"{label}\", \"status\": \"not_installed\"}}");
                    return std::process::ExitCode::FAILURE;
                }
                service_manager::ServiceStatus::Running => {
                    println!("{{\"label\": \"{label}\", \"status\": \"running\"}}")
                }
                service_manager::ServiceStatus::Stopped(opt_s) => match opt_s {
                    None => println!("{{\"label\": \"{label}\", \"status\": \"stopped\"}}"),
                    Some(s) => println!(
                        "{{\"label\": \"{label}\", \"status\": \"stopped\", \"details\": \"{s}\"}}"
                    ),
                },
            }
        }
        Some(Commands::Start { label }) => {
            match manager.start(service_manager::ServiceStartCtx {
                label: label.parse().unwrap(),
            }) {
                Ok(_) => {
                    println!("{{\"label\": \"{label}\", \"status\": \"started\"}}")
                }
                Err(e) => panic!("{}", e),
            }
        }
        Some(Commands::Stop { label }) => {
            match manager.stop(service_manager::ServiceStopCtx {
                label: label.parse().unwrap(),
            }) {
                Ok(_) => {
                    println!("{{\"label\": \"{label}\", \"status\": \"stopped\"}}")
                }
                Err(e) => panic!("{}", e),
            }
        }
    };
    std::process::ExitCode::SUCCESS
}
