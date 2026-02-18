use crate::domain::entities::{FileStats, Recommendation};
use anyhow::Result;
use std::path::Path;
use std::sync::mpsc::Sender;

pub trait Scanner: Send + Sync {
    fn scan(&self, root: &Path, tx: Sender<ScanEvent>);
}

#[derive(Debug, Clone)]
pub enum ScanEvent {
    Progress {
        total_size: u64,
        files_scanned: usize,
    },
    NewEntry(FileStats),
    Complete,
    Error(String),
}

pub trait Cleaner: Send + Sync {
    fn delete_item(&self, path: &Path) -> Result<u64>;
}

pub trait Analyzer: Send + Sync {
    fn analyze(&self) -> Result<Option<Recommendation>>;
    fn prune(&self) -> Result<()>; // Should probably be generalized or handled via command pattern
}
