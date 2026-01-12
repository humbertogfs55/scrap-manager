use anyhow::{Context, Result};
use simplelog::*;
use std::fs::File;
use std::path::Path;

pub fn init(log_dir: &Path) -> Result<()> {
    let log_file = log_dir.join("scrap-manager.log");

    let file = File::create(&log_file)
        .with_context(|| format!("failed to create log file at {:?}", log_file))?;

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        file,
    )])
    .context("failed to initialize logger")?;

    Ok(())
}
