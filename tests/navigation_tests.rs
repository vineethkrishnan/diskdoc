use diskdoc::application::app::App;
use diskdoc::domain::entities::{FileStats, FileType};
use std::path::PathBuf;
use std::time::SystemTime;

mod test_utils;
use test_utils::{MockAnalyzer, MockCleaner};

fn create_entry(path: &str, is_dir: bool) -> FileStats {
    FileStats {
        path: PathBuf::from(path),
        size: 100,
        is_dir,
        modified: SystemTime::UNIX_EPOCH,
        children_count: None,
        file_type: FileType::Normal,
    }
}

#[test]
fn test_navigation() {
    let mut app = App::new(
        "/root".to_string(),
        Box::new(MockCleaner::new()),
        Box::new(MockAnalyzer::new()),
    );

    // Setup file structure:
    // /root/file1
    // /root/dir1
    // /root/dir1/file2

    app.files.push(create_entry("/root/file1", false));
    app.files.push(create_entry("/root/dir1", true));
    app.files.push(create_entry("/root/dir1/file2", false));

    // Initial view should show file1 and dir1
    let current = app.get_current_files();
    assert_eq!(current.len(), 2);

    // Select dir1 (index 1, since sorting might affect it, let's force order or check path)
    // By default sort is Desc size. All size 100. Stable sort?
    // Let's rely on finding index.
    let dir_index = current.iter().position(|f| f.is_dir).unwrap();
    app.selection = dir_index;

    // Enter directory
    app.enter_dir();
    assert_eq!(app.current_path, PathBuf::from("/root/dir1"));
    assert_eq!(app.selection, 0);

    // View should now show file2
    let current = app.get_current_files();
    assert_eq!(current.len(), 1);
    assert_eq!(current[0].path, PathBuf::from("/root/dir1/file2"));

    // Go up
    app.go_up();
    assert_eq!(app.current_path, PathBuf::from("/root"));
    assert_eq!(app.selection, 0); // Logic resets to 0
}
