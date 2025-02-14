use anyhow::Result;
use language_server::run_server;
use log::LevelFilter;

fn main() -> Result<()> {
    simple_logging::log_to_file("test.log", LevelFilter::Info).unwrap();
    run_server()
}
