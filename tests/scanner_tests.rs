use diskdoc::domain::entities::{FileStats, FileType};
use std::path::PathBuf;
use std::time::SystemTime;

#[test]
fn test_file_stats_creation() {
    let path = PathBuf::from("test_file.txt");
    let stats = FileStats {
        path: path.clone(),
        size: 1024,
        is_dir: false,
        modified: SystemTime::UNIX_EPOCH,
        children_count: None,
        file_type: FileType::Normal,
    };

    assert_eq!(stats.path, PathBuf::from("test_file.txt"));
}
