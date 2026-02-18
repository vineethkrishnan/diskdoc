use crate::domain::entities::FileStats;
use crate::domain::ports::ScanEvent;
use jwalk::{Parallelism, WalkDir};
use std::path::Path;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::SystemTime;

pub fn start_scan(root: &Path, tx: Sender<ScanEvent>) {
    let root_path = root.to_path_buf();

    thread::spawn(move || {
        let walk = WalkDir::new(&root_path)
            .skip_hidden(false)
            .sort(true)
            .parallelism(Parallelism::RayonNewPool(4));

        let heuristics_engine = crate::infrastructure::scanner::heuristics::HeuristicsEngine::new();

        for entry in walk {
            match entry {
                Ok(dir_entry) => {
                    let path = dir_entry.path();
                    let metadata = dir_entry.metadata();

                    if let Ok(meta) = metadata {
                        let size = meta.len();
                        let modified = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
                        let is_dir = dir_entry.file_type.is_dir();
                        let file_type = heuristics_engine.analyze(&path, is_dir);

                        let stats = FileStats {
                            path,
                            size,
                            is_dir,
                            modified,
                            children_count: None,
                            file_type,
                        };

                        if tx.send(ScanEvent::NewEntry(stats)).is_err() {
                            break; // Receiver dropped
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(ScanEvent::Error(e.to_string()));
                }
            }
        }

        let _ = tx.send(ScanEvent::Complete);
    });
}
