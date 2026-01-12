mod app;
mod util;

use anyhow::Result;
use util::{logging, paths::AppPaths};

fn main() {
    if let Err(err) = run() {
        eprintln!("fatal error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let paths = AppPaths::init()?;

    logging::init(&paths.log_dir)?;

    log::info!("application starting");

    app::run(paths)
}
