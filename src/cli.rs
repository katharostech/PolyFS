use std::ffi::OsString;
use std::io;

use clap::{App, AppSettings, Arg, Shell, SubCommand};

pub mod config;

/// Run the CLI
pub fn run() {
    // Parse commandline arguments
    let args = parse_arguments(std::env::args_os()).unwrap_or_else(|err| {
        err.exit();
    });

    log::trace!("{:#?}", args);

    // Run selected subcommand
    match args.subcommand() {
        // Output shell completion script
        ("completion", Some(args)) => {
            output_completion(clap::value_t_or_exit!(args.value_of("shell"), clap::Shell));
        }

        ("config", Some(args)) => config::run(args),

        // TODO: Implement all the commands!
        (_, Some(_args)) => log::error!("Command not implemented"),

        _ => panic!(
            "Argument parsing should have shown help message and exited before getting here."
        ),
    }
}

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

pub fn parse_arguments<'a, I, T>(args: I) -> clap::Result<clap::ArgMatches<'a>>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    get_cli().get_matches_from_safe(args)
}

pub fn output_completion(shell: clap::Shell) {
    get_cli().gen_completions_to("polyfs", shell, &mut io::stdout());
}
