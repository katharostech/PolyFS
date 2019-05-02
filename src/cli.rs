//! Components of the command line interface.

use std::ffi::OsString;
use std::io;

use clap::{App, AppSettings, Arg, Shell, SubCommand};

pub mod config;

#[macro_use]
// The allow(missing_docs) is necessary because of the arg_enum! macro that
// 
#[allow(missing_docs)]
pub mod types;
pub use types::*;

/// Run the CLI.
pub fn run() {
    log::debug!("Starting CLI");

    // Parse command line arguments
    let args = parse_arguments(std::env::args_os()).unwrap_or_else(|err| {
        err.exit();
    });

    log::trace!("args = {:#?}", args);

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
        .arg(Arg::with_name("config_file")
            .help(
"PolyFS config file. Must be in YAML 1.2 format. Can be conveniently created \
and modified with the `config` subcommand."
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

        // `config` subcommand
        .subcommand(config::get_cli())

        .subcommand(SubCommand::with_name("mount")
            .about("Mount a backend as a filesystem")
            .arg(Arg::with_name("read_only")
                .long("read-only")
                .short("r")
                .help("Mount the filesystem as read-only"))
            .arg(Arg::with_name("mountpoint")
                    .help("location to mount the filesystem")
                    .required(true)))

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
