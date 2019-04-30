use clap::{App, Arg, SubCommand};

/// Get CLI for the `sqlite` subcommand
#[rustfmt::skip]
pub fn get_cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("sqlite")
        .about("Configure Sqlite backend")
        .arg(Arg::with_name("db")
            .help("Path to the Sqlite database")
            .required(true))
}
