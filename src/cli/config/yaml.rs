//! Utilities for handling yaml configuration

use crate::cli::{CliError, CliResult};
use crate::try_to;

use std::fs;
use std::io::Write;
use std::path::Path;

use clap::ArgMatches;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

/// Get the YAML configuration given an argument match list
pub fn get_config<'a>(args: &ArgMatches<'a>) -> CliResult<Yaml> {
    let config_path = args
        .value_of("config_file")
        .expect("Couldn't read config file from commandline args.");

    let mut config;

    if Path::new(&config_path).exists() {
        // Load config file
        log::debug!("Loading configuration file from: {}", &config_path);

        let content = try_to!(
            fs::read_to_string(&config_path),
            "Could not read config file."
        );
        config = try_to!(
            YamlLoader::load_from_str(&content),
            "Could not parse YAML config file."
        )[0] // We load the first doc in the YAML stream and ignore the rest
        .clone();
    } else {
        // Config file doesn't exist, create it
        log::debug!("Creating config file: {}", &config_path);

        let mut file = try_to!(
            fs::File::create(&config_path),
            "Could not create config file."
        );

        let mut yaml_string = String::new();
        let mut emitter = YamlEmitter::new(&mut yaml_string);
        emitter
            .dump(&get_default_config())
            // This should always work; panic if it doesn't.
            .expect("Couldn't dump yaml config.");

        try_to!(
            file.write_all(yaml_string.as_bytes()),
            "Couldn't write to config file."
        );

        config = get_default_config();
    }

    log::trace!("Loaded configuration: {:#?}", config);

    Ok(config)
}

fn get_default_config() -> Yaml {
    YamlLoader::load_from_str(include_str!("default-config.yml"))
        .expect("Error in default-config.yml, couldn't parse yaml")[0]
        .clone()
}
