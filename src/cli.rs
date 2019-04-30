use std::ffi::OsString;
use std::error::Error;
use std::{io, fmt};

use clap::{App, AppSettings, Arg, ArgMatches, Shell, SubCommand};

pub mod config;

/// A container for the gobal CLI args and the current submodule args.
///
/// This is a convenient way to pass the arguments that a subcommand are going
/// to need.
pub struct ArgSet<'a> {
    /// The global CLI argument matches.
    global: &'a ArgMatches<'a>,
    /// The argument matches for the current subcommand.
    sub: &'a ArgMatches<'a>,
}

pub type CliResult<T> = Result<T, CliError>;

/// A CLI Error.
#[derive(Debug)]
pub struct CliError {
    /// Should describe what the program was trying to do and could not.
    message: String,
    /// The actual Error that ocurred when attempting to perform the operation
    /// described by the `message`.
    cause: Option<Box<(dyn Error)>>
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.source() {
            Some(cause) => write!(f, "{}\nCaused by: {}", self.message, cause),
            None => write!(f, "{}", self.message)
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.cause {
            Some(e) => Some(e.as_ref()),
            None => None
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError {
            message: String::from("IO Error:"),
            cause: Some(Box::new(error))
        }
    }
}

/// Run the CLI.
pub fn run() {
    log::debug!("Starting CLI");

    // Parse commandline arguments
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
            config::run(ArgSet { global: &args, sub })
                .unwrap_or_else(|e| {
                    log::error!("{}", e);
                    std::process::exit(1);
                });
        },

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
        .author("Katharos Technology <https://katharostech.com>\n")
        .about("A FUSE filesytem for many backends.")
        .long_about(
"A FUSE filesystem for many backends.

PolyFS allows you to mount a filesystem built on a key-value store and \
a metadata store. Multiple key-value and metadata stores are supported ( not \
at the same time ).
            
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

/// Parse given arguments as they would be from the commandline.
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
