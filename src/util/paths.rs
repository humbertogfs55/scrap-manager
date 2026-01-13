use anyhow::{Context, Result};
use directories_next::ProjectDirs;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppPaths {
    pub data_dir: PathBuf,
    pub db_dir: PathBuf,
    pub log_dir: PathBuf,
}

impl AppPaths {
    pub fn init() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("local", "scrap", "scrap-manager")
            .context("failed to resolve project directories")?;

        let data_dir = proj_dirs.data_dir().to_path_buf();
        let log_dir = data_dir.join("logs");
        let db_dir = data_dir.join("scrap_manager.db");

        fs::create_dir_all(&data_dir).context("failed to create data directory")?;

        fs::create_dir_all(&log_dir).context("failed to create log directory")?;

        Ok(Self {
            data_dir,
            db_dir,
            log_dir,
        })
    }
}
