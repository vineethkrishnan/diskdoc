use crate::domain::ports::Cleaner;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub struct FsCleaner;

impl FsCleaner {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FsCleaner {
    fn default() -> Self {
        Self::new()
    }
}

impl Cleaner for FsCleaner {
    fn delete_item(&self, path: &Path) -> Result<u64> {
        let metadata = fs::metadata(path).context("Failed to read metadata")?;
        let size = metadata.len();

        if path.is_dir() {
            fs::remove_dir_all(path).context("Failed to remove directory")?;
        } else {
            fs::remove_file(path).context("Failed to remove file")?;
        }

        Ok(size)
    }
}
