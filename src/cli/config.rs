//! `config` subcommand. Also contains utilities for accessing commandline
//! config.

use crate::app::config::{AppConfig};
use crate::PolyfsResult;
use crate::cli::{ArgSet, ConfigFormat};
use crate::try_to;

use clap::{value_t, App, ArgMatches, SubCommand};

use std::fs;
use std::io::Write;
use std::path::Path;

pub mod dual;
pub mod kv;
pub mod meta;
pub mod default;

/// Run `config` subcommand
pub fn run<'a>(args: ArgSet) -> PolyfsResult<()> {
    log::debug!("Running `config` subcommand");

    match args.sub.subcommand() {
        ("kv", Some(sub)) => kv::run(ArgSet {
            global: args.global,
            sub,
        })?,

        ("meta", Some(sub)) => meta::run(ArgSet {
            global: args.global,
            sub,
        })?,

        ("dump", Some(_)) => {
            // Dump configuration in debug format
            let config = load_config(args.global)?;
            log::debug!("Dumping config to standard out in debug format");
            println!("{:#?}", config);
        }

        ("default", Some(sub)) => default::run(ArgSet {
            global: args.global,
            sub,
        })?,

        _ => panic!(
            "Unimplemented command or failure to show help message when lacking a subcommand."
        ),
    }

    Ok(())
}

/// Get CLI for the `config` subcommand
pub fn get_cli<'a, 'b>() -> App<'a, 'b> {
    let mut command = SubCommand::with_name("config")
        .about("Create or update PolyFS config file")
        .subcommand(kv::get_cli())
        .subcommand(meta::get_cli())
        .subcommand(default::get_cli());

    if std::env::var("POLYFS_DEBUG").is_ok() {
        command = command.subcommand(SubCommand::with_name("dump")
            .about("Dump loaded configuration in debug format. ( a POLYFS_DEBUG command )"));
    }

    command
}

/// Load app config based provided command line arguments.
pub fn load_config<'a>(args: &ArgMatches<'a>) -> PolyfsResult<AppConfig> {
    let config_format = value_t!(args, "config_format", ConfigFormat)
        .expect("Couldn't parse config format argument");
    let config_path = args
        .value_of("config_file")
        .expect("Required config file argumetn doesn't exist");

    let mut config;

    if Path::new(&config_path).exists() {
        // Load config file
        log::debug!("Loading configuration file from: {}", &config_path);

        let content = try_to!(
            fs::read_to_string(&config_path),
            "Could not read config file."
        );

        config = deserialize_config(&content, config_format)?;
    } else {
        // Config file doesn't exist, create it with default settings
        log::debug!("Creating config file: {}", &config_path);

        let mut file = try_to!(
            fs::File::create(&config_path),
            "Could not create config file."
        );

        config = AppConfig::default();
        let serialized = serialize_config(&config, config_format)?;

        try_to!(
            file.write_all(serialized.as_bytes()),
            "Couldn't write to config file."
        );
    }

    log::trace!("Loaded configuration: {:#?}", config);

    Ok(config)
}

/// Save config file with the provide config
pub fn save_config<'a>(args: &ArgMatches<'a>, config: &AppConfig) -> PolyfsResult<()> {
    let config_format = value_t!(args, "config_format", ConfigFormat)
        .expect("Couldn't parse config format argument");
    let config_path = args
        .value_of("config_file")
        .expect("Required config file argument doesn't exist");

    log::debug!("Saving config to file: {}", config_path);

    try_to!(
        fs::write(config_path, serialize_config(config, config_format)?),
        "Could not write config file."
    );

    log::trace!("Saved configuration: {:#?}", config);

    Ok(())
}

/// Serialize an `AppConfig` object for a given config format.
pub fn serialize_config(config: &AppConfig, format: ConfigFormat) -> PolyfsResult<String> {
    Ok(match format {
        ConfigFormat::yaml => try_to!(serde_yaml::to_string(config), "Could not serialize config"),
        ConfigFormat::json => try_to!(
            serde_json::to_string_pretty(config),
            "Could not serialize config"
        ),
    })
}

/// Deserialize a string representation in a given format to an `AppConfig` object.
fn deserialize_config(config: &str, format: ConfigFormat) -> PolyfsResult<AppConfig> {
    Ok(match format {
        ConfigFormat::yaml => try_to!(
            serde_yaml::from_str(config),
            "Could not deserialize config"
        ),
        ConfigFormat::json => try_to!(
            serde_json::from_str(config),
            "Could not deserialize config"
        ),
    })
}
