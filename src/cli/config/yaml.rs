//! Utilities for handling yaml configuration

use std::fs;
use std::io::Write;
use std::iter::FromIterator;
use std::path::Path;

use crate::cli::{CliError, CliResult};
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
        let content;
        match fs::read_to_string(&config_path) {
            Ok(data) => content = data,
            Err(e) => {
                return Err(CliError {
                    message: String::from("Could not read config file."),
                    cause: Some(Box::new(e)),
                })
            }
        }
        match YamlLoader::load_from_str(&content) {
            Ok(yaml_docs) => {
                config = yaml_docs[0].clone();
            }
            Err(e) => {
                return Err(CliError {
                    message: String::from("Could not parse yaml config file."),
                    cause: Some(Box::new(e)),
                })
            }
        }
    } else {
        // Config file doesn't exist, create it
        let mut file = fs::File::create(&config_path)?;

        let mut yaml_string = String::new();
        let mut emitter = YamlEmitter::new(&mut yaml_string);
        // TODO Create macro for handling these errors easiliy without panicking.
        emitter
            .dump(&get_default_config())
            .expect("Couldn't write yaml config");

        file.write_all(yaml_string.as_bytes())
            .expect("Couldn't write config file");

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
