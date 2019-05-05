//! Components of the command line interface.

use std::ffi::OsString;
use std::io;

use clap::{App, AppSettings, Arg, ArgMatches, Shell, SubCommand};

use crate::log::{LoggingConfig, setup_logging};

// Subcommands
pub mod config;
pub mod mount;

/// This is a convenient way to pass the arguments that a subcommand are going
/// to need.
pub struct ArgSet<'a> {
    /// The global CLI argument matches.
    pub global: &'a ArgMatches<'a>,
    /// The argument matches for the current subcommand.
    pub sub: &'a ArgMatches<'a>,
}

// It is impossible to fully document enums made with the `arg_enum` macro, so
// put them in a module and allow missing docs on that module.
#[allow(missing_docs)]
mod arg_enums {
    use clap::arg_enum;

    arg_enum! {
        /// A file format supported for the PolyFS config file
        #[allow(non_camel_case_types, missing_docs)]
        #[derive(PartialEq, Debug)]
        pub enum ConfigFormat {
            yaml,
            json,
        }
    }
}

pub use arg_enums::ConfigFormat;

/// Run the CLI.
pub fn run() {
    // Parse command line arguments
    let args = parse_arguments(std::env::args_os()).unwrap_or_else(|err| {
        err.exit();
    });

    // Setup logging
    let mut log_config = LoggingConfig::default();

    fn map_log_level(level: &str) -> log::LevelFilter {
        match level {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            unknown_level => {
                eprintln!("Warning: Ignoring unrecognized warning level, '{}'. \
                           Setting log level to 'warn'.", unknown_level);
                log::LevelFilter::Warn
            }
        }
    }

    log_config.log_level = match args.value_of("log_level") {
        Some(level) => map_log_level(level),
        None => {
            map_log_level(&std::env::var("POLYFS_LOG_LEVEL").unwrap_or("warn".into()))
        }
    };

    log_config.quiet = args.is_present("quiet");
    log_config.syslog = args.is_present("syslog");
    log_config.log_file = args.value_of("log_file").map(|s| {String::from(s)});

    setup_logging(log_config).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1)
    });

    log::debug!("Starting CLI");

    log::trace!("{:#?}", args);

    // Run selected subcommand
    match args.subcommand() {
        // Output shell completion script
        ("completion", Some(args)) => {
            output_completion(clap::value_t_or_exit!(args.value_of("shell"), clap::Shell));
        }

        ("config", Some(sub)) => {
            config::run(ArgSet { global: &args, sub }).unwrap_or_else(|e| {
                log::error!("{}", e);
                std::process::exit(1);
            });
            log::info!(
                "Successfully updated config file: {}",
                args.value_of("config_file")
                    .expect("Could not parse config-file argument.")
            );
        },

        ("mount", Some(sub)) => {
            mount::run(ArgSet { global: &args, sub }).unwrap_or_else(|e| {
                log::error!("{}", e);
                std::process::exit(1);
            });
        }

        _ => panic!(
            "Unimplemented command or failure to show help message when lacking a subcommand."
        ),
    }
}

/// Get CLI application structure.
#[rustfmt::skip]
pub fn get_cli() -> App<'static, 'static> {
    App::new("PolyFS")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("A FUSE filesytem for many backends.")
        .long_about(
"A FUSE filesystem for many backends.

PolyFS allows you to mount a filesystem built on a key-value store and \
a metadata store. Multiple key-value and metadata stores are supported.
            
Usually you will run `polyfs config kv` and `polyfs config meta` to create the \
config file with your connection information, followed  by `polyfs mount` to \
mount the filesystem."
        )
        .global_setting(AppSettings::ColoredHelp)
        .setting(AppSettings::SubcommandRequiredElseHelp)

        // Global arguments
        .arg(Arg::with_name("config_file")
            .help(
"PolyFS config file. Must be in the format specified by the --config-format. \
Can be conveniently created and modified with the `config` subcommand."
            )
            .long("config-file")
            .default_value("polyfs.yml")
            .short("c"))
        .arg(Arg::with_name("config_format")
            .help("The configuration file format.")
            .long("config-format")
            .short("F")
            .possible_values(&ConfigFormat::variants())
            .case_insensitive(true)
            .default_value("yaml"))
        .arg(Arg::with_name("log_level")
            .help("Logging level. May also be configured with the POLYFS_LOG_LEVEL \
                   environment variable.")
            .long("log-level")
            .short("L")
            .value_name("level")
            .possible_values(&vec!["trace", "debug", "info", "warn", "error"]))
        .arg(Arg::with_name("log_file")
            .help("File to log to.")
            .long("log-file")
            .short("l")
            .value_name("file"))
        .arg(Arg::with_name("syslog")
            .help("Log to syslog")
            .long("syslog"))
        .arg(Arg::with_name("quiet")
            .help("Do not log to stderr")
            .long("quiet")
            .short("q"))

        // `config` subcommand
        .subcommand(config::get_cli())

        .subcommand(mount::get_cli())

        .subcommand(SubCommand::with_name("completion")
            .about("Output shell completion scripts")
            .arg(Arg::with_name("shell")
                .help("The shell to generate completion script for.")
                .required(true)
                .possible_values(&Shell::variants().to_vec())))
}

/// Parse given arguments as they would be from the command line.
pub fn parse_arguments<'a, I, T>(args: I) -> clap::Result<clap::ArgMatches<'a>>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    get_cli().get_matches_from_safe(args)
}

/// Print CLI shell completion script for the given shell to standard out.
pub fn output_completion(shell: clap::Shell) {
    get_cli().gen_completions_to("polyfs", shell, &mut io::stdout());
}
