use diskdoc::application::app::{App, AppMode};
use diskdoc::domain::entities::{FileStats, FileType, Recommendation, RecommendationCategory};
mod test_utils;
use std::path::PathBuf;
use std::time::SystemTime;
use test_utils::{MockAnalyzer, MockCleaner};

fn create_file_stat(name: &str, size: u64, file_type: FileType) -> FileStats {
    FileStats {
        path: PathBuf::from(name),
        size,
        is_dir: false,
        modified: SystemTime::UNIX_EPOCH,
        children_count: None,
        file_type,
    }
}

#[test]
fn test_dashboard_aggregation() {
    let mut app = App::new(
        "/test".to_string(),
        Box::new(MockCleaner::new()),
        Box::new(MockAnalyzer::new()),
    );

    // Add some files
    app.files
        .push(create_file_stat("app.log", 1000, FileType::Log));
    app.files
        .push(create_file_stat("error.log", 2000, FileType::Log));
    app.files
        .push(create_file_stat("cache.bin", 500, FileType::Cache));
    app.files
        .push(create_file_stat("normal.txt", 100, FileType::Normal));

    // Scan dashboard
    app.scan_dashboard();

    // Check Logs
    let log_rec = app
        .recommendations
        .iter()
        .find(|r| r.category == RecommendationCategory::Log)
        .unwrap();
    assert_eq!(log_rec.size, 3000);
    assert!(log_rec.description.contains("2 files"));

    // Check Cache
    let cache_rec = app
        .recommendations
        .iter()
        .find(|r| r.category == RecommendationCategory::Cache)
        .unwrap();
    assert_eq!(cache_rec.size, 500);
}

#[test]
fn test_dashboard_cleanup_flow() {
    let mut app = App::new(
        "/test".to_string(),
        Box::new(MockCleaner::new()),
        Box::new(MockAnalyzer::new()),
    );
    // Mock recommendations
    app.recommendations.push(Recommendation {
        category: RecommendationCategory::Log,
        description: "Logs".to_string(),
        size: 100,
        path: None,
        action_command: None,
    });

    app.mode = AppMode::Dashboard;

    // Select first one
    app.recommendation_selection = 0;

    // Request clean
    app.request_clean_recommendation();
    assert_eq!(app.mode, AppMode::DashboardCleanupConfirmation);

    // Cancel
    app.cancel_clean();
    assert_eq!(app.mode, AppMode::Dashboard);

    // Request again
    app.request_clean_recommendation();

    // Confirm (should call logic, but without files it does nothing or panics if not handled)
    // We haven't populated app.files matching types, so it should be safe.
    app.confirm_clean_recommendation();
    assert_eq!(app.mode, AppMode::Dashboard);
}
