use clap::{App, ArgMatches, SubCommand};

pub mod kv;
pub mod meta;

pub fn run<'a>(args: &ArgMatches<'a>) {
    log::info!("Generating config");
    log::debug!("Config: {:#?}", args);
}

pub fn get_cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("config")
}
