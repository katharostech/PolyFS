fn main() {
    configure_logging();

    polyfs::cli::run();
}

fn configure_logging() {
    env_logger::init_from_env(env_logger::Env::new().filter("POLYFS_LOG_LEVEL"));
}
