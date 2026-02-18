pub mod heuristics;
pub mod walker;

use crate::domain::ports::{ScanEvent, Scanner};
use std::path::Path;
use std::sync::mpsc::Sender;

pub struct FsScanner;

impl FsScanner {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FsScanner {
    fn default() -> Self {
        Self::new()
    }
}

impl Scanner for FsScanner {
    fn scan(&self, root: &Path, tx: Sender<ScanEvent>) {
        walker::start_scan(root, tx);
    }
}
