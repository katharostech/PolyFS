//! Utilities for handling yaml configuration

use crate::cli::{CliError, CliResult};
use crate::try_to;

use std::fs;
use std::io::Write;
use std::iter::FromIterator;
use std::path::Path;

use clap::ArgMatches;
use linked_hash_map::LinkedHashMap;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

/// Get the YAML configuration given an argument match list
pub fn get_config<'a>(args: &ArgMatches<'a>) -> CliResult<Yaml> {
    let config_path = args
        .value_of("config_file")
        .expect("Couldn't read config file from commandline args");

    let mut config;

    if Path::new(&config_path).exists() {
        // Load config file
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
        let mut file = try_to!(fs::File::create(&config_path), "Create config file.");

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

    Ok(config)
}

fn get_default_config() -> Yaml {
    Yaml::Hash(LinkedHashMap::from_iter(vec![
        (
            Yaml::String("key-value".to_string()),
            Yaml::Hash(LinkedHashMap::from_iter(vec![(
                Yaml::String("backend".to_string()),
                Yaml::String("sqlite".to_string()),
            )])),
        ),
        (
            Yaml::String("metadata".to_string()),
            Yaml::Hash(LinkedHashMap::from_iter(vec![(
                Yaml::String("backend".to_string()),
                Yaml::String("sqlite".to_string()),
            )])),
        ),
    ]))
}
