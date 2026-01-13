mod app;
mod db;
mod domain;
mod ui;
mod util;

use anyhow::{Context, Result};
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

    let conn = db::connect(&paths.db_dir).context("failed to open database")?;

    db::init(&conn).context("failed to initialize database")?;
    log::info!("opened database");
    log::info!(
        "application starting, Database ready: backend= sqlite path={}",
        paths.db_dir.display()
    );

    //eprintln!("app started log file in: {}", paths.log_dir.display());
    app::run(paths)
}
