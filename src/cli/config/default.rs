//! Create default configuration file
use std::path::Path;

use crate::app::config::AppConfig;
use crate::try_to;
use crate::cli::{ArgSet, CliResult};
use crate::cli::config::save_config;

use clap::{App, Arg, SubCommand};

/// Get the `default` subcommand
pub fn get_cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("default")
        .about("Save config file with default values")
        .arg(Arg::with_name("force")
            .help("Do not prompt if config file already exists")
            .long("force")
            .short("f"))
}

/// Save the default configuration
pub fn run(args: ArgSet) -> CliResult<()> {
    log::debug!("Running `default` subcommand");

    let force = args.sub.is_present("force");

    let config_path = args.global
        .value_of("config_file")
        .expect("Required config file argument doesn't exist");

    let write_config = || {
        save_config(args.global, &AppConfig::default())?;

        Ok(())
    };

    if Path::new(&config_path).exists() {
        if force {
            write_config()?;
        } else {
            eprintln!(
                "Config file, '{}', exists, overwrite? Type \"yes\" to confirm:",
                config_path
            );

            let mut prompt_result = String::new();
            try_to!(
                std::io::stdin().read_line(&mut prompt_result),
                "Could not readline for prompt"
            );

            if &prompt_result.trim() == &"yes" {
                write_config()?;
            } else {
                log::warn!("Not applying config");
                std::process::exit(0);
            }
        }
    } else {
        write_config()?;
    }

    Ok(())
}
