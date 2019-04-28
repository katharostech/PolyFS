
fn main() {
    configure_logging();

    let _arg_matches = parse_arguments();

    polyfs::run();
}

fn configure_logging() {
    env_logger::init_from_env(env_logger::Env::new().filter("POLYFS_LOG_LEVEL"));
}

#[rustfmt::skip]
fn parse_arguments<'a>() -> clap::ArgMatches<'a> {
    use clap::{App, AppSettings, Arg, SubCommand};

    App::new("PolyFS")
        .version(clap::crate_version!())
        .author("Katharos Technology <https://katharostech.com>")
        .about("A FUSE filesytem for many backends.")
        .global_setting(AppSettings::ColoredHelp)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("mount")
            .about("mount a filesystem")
            .arg(Arg::with_name("read-only")
                .long("read-only")
                .short("r")
                .help("Mount the filesystem as read-only"))
            .arg(Arg::with_name("mountpoint")
                    .help("location to mount the filesystem")
                    .required(true)))
        .get_matches()
}

