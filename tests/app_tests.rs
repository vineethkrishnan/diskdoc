use diskdoc::application::app::{App, AppMode};
use diskdoc::domain::entities::{FileStats, FileType, SortOrder};
use std::path::PathBuf;
use std::time::SystemTime;

mod test_utils;
use test_utils::{MockAnalyzer, MockCleaner};

fn setup_app() -> App {
    App::new(
        "/test".to_string(),
        Box::new(MockCleaner::new()),
        Box::new(MockAnalyzer::new()),
    )
}
fn create_file_stat(name: &str, size: u64) -> FileStats {
    FileStats {
        path: PathBuf::from(name),
        size,
        is_dir: false,
        modified: SystemTime::UNIX_EPOCH,
        children_count: None,
        file_type: FileType::Normal,
    }
}

#[test]
fn test_app_sorting() {
    let mut app = setup_app();

    app.files.push(create_file_stat("small", 100));
    app.files.push(create_file_stat("large", 1000));
    app.files.push(create_file_stat("medium", 500));

    // Default is Descending
    assert_eq!(app.sort_order, SortOrder::Desc);

    app.toggle_sort(); // Switch to Ascending
    assert_eq!(app.sort_order, SortOrder::Asc);

    // Verify Ascending order
    assert_eq!(app.files[0].size, 100);
    assert_eq!(app.files[1].size, 500);
    assert_eq!(app.files[2].size, 1000);

    app.toggle_sort(); // Becomes Desc
    assert_eq!(app.sort_order, SortOrder::Desc);

    // Verify Descending order
    assert_eq!(app.files[0].size, 1000);
    assert_eq!(app.files[1].size, 500);
    assert_eq!(app.files[2].size, 100);

    app.toggle_sort();
    assert_eq!(app.sort_order, SortOrder::Asc);
    assert_eq!(app.files[0].size, 100);
}

#[test]
fn test_delete_confirmation_flow() {
    let mut app = setup_app();
    app.files.push(create_file_stat("/test/file1", 100));
    app.mode = AppMode::Browsing;

    // Request delete
    app.request_delete();
    assert_eq!(app.mode, AppMode::DeleteConfirmation);
    assert_eq!(app.item_to_delete, Some(PathBuf::from("/test/file1")));

    // Cancel delete
    app.cancel_delete();
    assert_eq!(app.mode, AppMode::Browsing);
    assert_eq!(app.item_to_delete, None);

    // Request again
    app.request_delete();
    assert_eq!(app.mode, AppMode::DeleteConfirmation);
}
