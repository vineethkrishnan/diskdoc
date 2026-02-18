use crate::domain::entities::{
    FileStats, FileType, Recommendation, RecommendationCategory, SortOrder,
};
use crate::domain::ports::{Analyzer, Cleaner, ScanEvent};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Scanning,
    Browsing,
    DeleteConfirmation,
    Dashboard,
    DashboardCleanupConfirmation,
    About,
}

pub struct App {
    pub current_path: PathBuf,
    pub mode: AppMode,
    pub scan_receiver: Option<Receiver<ScanEvent>>,
    pub files: Vec<FileStats>,
    pub total_size: u64,
    pub scanned_count: usize,
    pub is_scanning: bool,
    pub sort_order: SortOrder,
    pub selection: usize,
    pub item_to_delete: Option<PathBuf>,
    pub recommendations: Vec<Recommendation>,
    pub recommendation_selection: usize,

    // Dependencies
    pub cleaner: Box<dyn Cleaner>,
    pub analyzer: Box<dyn Analyzer>,
}

impl App {
    pub fn new(path: String, cleaner: Box<dyn Cleaner>, analyzer: Box<dyn Analyzer>) -> App {
        App {
            current_path: PathBuf::from(path),
            mode: AppMode::Scanning,
            scan_receiver: None,
            files: Vec::new(),
            total_size: 0,
            scanned_count: 0,
            is_scanning: true,
            sort_order: SortOrder::Desc,
            selection: 0,
            item_to_delete: None,
            recommendations: Vec::new(),
            recommendation_selection: 0,
            cleaner,
            analyzer,
        }
    }

    pub fn on_tick(&mut self) {
        if let Some(rx) = &self.scan_receiver {
            let mut needs_sort = false;
            for _ in 0..100 {
                match rx.try_recv() {
                    Ok(event) => match event {
                        ScanEvent::NewEntry(stats) => {
                            self.total_size += stats.size;
                            self.files.push(stats);
                            self.scanned_count += 1;
                            needs_sort = true;
                        }
                        ScanEvent::Progress {
                            total_size: _,
                            files_scanned: _,
                        } => {}
                        ScanEvent::Complete => {
                            self.is_scanning = false;
                            self.mode = AppMode::Browsing;
                            needs_sort = true;
                        }
                        ScanEvent::Error(err) => {
                            eprintln!("Scan error: {}", err);
                        }
                    },
                    Err(_) => break,
                }
            }

            if needs_sort && !self.is_scanning {
                self.sort_files();
            }
        }
    }

    pub fn toggle_sort(&mut self) {
        self.sort_order = match self.sort_order {
            SortOrder::Desc => SortOrder::Asc,
            SortOrder::Asc => SortOrder::Desc,
        };
        self.sort_files();
    }

    fn sort_files(&mut self) {
        self.files.sort_by(|a, b| match self.sort_order {
            SortOrder::Desc => b.size.cmp(&a.size),
            SortOrder::Asc => a.size.cmp(&b.size),
        });
    }

    pub fn enter_dir(&mut self) {
        let current_files = self.get_current_files();
        if let Some(file) = current_files.get(self.selection) {
            if file.is_dir {
                self.current_path = file.path.clone();
                self.selection = 0;
            }
        }
    }

    pub fn go_up(&mut self) {
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
            self.selection = 0;
        }
    }

    pub fn date_next(&mut self) {
        let max = self.get_current_files().len();
        if max > 0 {
            self.selection = (self.selection + 1) % max;
        }
    }

    pub fn date_prev(&mut self) {
        let max = self.get_current_files().len();
        if max > 0 {
            if self.selection == 0 {
                self.selection = max - 1;
            } else {
                self.selection -= 1;
            }
        }
    }

    pub fn get_current_files(&self) -> Vec<&FileStats> {
        self.files
            .iter()
            .filter(|f| {
                if let Some(parent) = f.path.parent() {
                    parent == self.current_path
                } else {
                    false
                }
            })
            .collect()
    }

    pub fn request_delete(&mut self) {
        let current_files = self.get_current_files();
        if let Some(file_stat) = current_files.get(self.selection) {
            self.item_to_delete = Some(file_stat.path.clone());
            self.mode = AppMode::DeleteConfirmation;
        }
    }

    pub fn confirm_delete(&mut self) {
        if let Some(path) = &self.item_to_delete {
            if let Ok(freed_size) = self.cleaner.delete_item(path) {
                let p = path.clone();
                if let Some(index) = self.files.iter().position(|f| f.path == p) {
                    self.files.remove(index);
                    self.total_size = self.total_size.saturating_sub(freed_size);

                    let new_len = self.get_current_files().len();
                    if self.selection >= new_len && new_len > 0 {
                        self.selection = new_len - 1;
                    }
                }
            } else {
                // TODO: Show error
            }
        }
        self.item_to_delete = None;
        self.mode = AppMode::Browsing;
    }

    pub fn cancel_delete(&mut self) {
        self.item_to_delete = None;
        self.mode = AppMode::Browsing;
    }

    pub fn scan_dashboard(&mut self) {
        self.recommendations.clear();
        self.recommendation_selection = 0;

        // 1. Logs
        let log_size: u64 = self
            .files
            .iter()
            .filter(|f| matches!(f.file_type, FileType::Log))
            .map(|f| f.size)
            .sum();

        if log_size > 0 {
            self.recommendations.push(Recommendation {
                category: RecommendationCategory::Log,
                description: format!(
                    "Log files found ({} files)",
                    self.files
                        .iter()
                        .filter(|f| matches!(f.file_type, FileType::Log))
                        .count()
                ),
                size: log_size,
                path: None,
                action_command: None,
            });
        }

        // 2. Caches
        let cache_size: u64 = self
            .files
            .iter()
            .filter(|f| {
                matches!(
                    f.file_type,
                    FileType::Cache
                        | FileType::NpmCache
                        | FileType::ComposerCache
                        | FileType::AptCache
                )
            })
            .map(|f| f.size)
            .sum();

        if cache_size > 0 {
            self.recommendations.push(Recommendation {
                category: RecommendationCategory::Cache,
                description: "Application & Package Caches".to_string(),
                size: cache_size,
                path: None,
                action_command: None,
            });
        }

        // 3. Docker (Use Analyzer)
        let docker_file_size: u64 = self
            .files
            .iter()
            .filter(|f| matches!(f.file_type, FileType::Docker))
            .map(|f| f.size)
            .sum();

        if let Ok(Some(rec)) = self.analyzer.analyze() {
            self.recommendations.push(rec);
        } else if docker_file_size > 0 {
            // Fallback
            self.recommendations.push(Recommendation {
                category: RecommendationCategory::Docker,
                description: "Docker Data (Volumes/Images) [CLI not available]".to_string(),
                size: docker_file_size,
                path: None,
                action_command: None,
            });
        }
    }

    pub fn dashboard_next(&mut self) {
        if !self.recommendations.is_empty() {
            self.recommendation_selection =
                (self.recommendation_selection + 1) % self.recommendations.len();
        }
    }

    pub fn dashboard_prev(&mut self) {
        if !self.recommendations.is_empty() {
            if self.recommendation_selection == 0 {
                self.recommendation_selection = self.recommendations.len() - 1;
            } else {
                self.recommendation_selection -= 1;
            }
        }
    }

    pub fn request_clean_recommendation(&mut self) {
        if !self.recommendations.is_empty() {
            self.mode = AppMode::DashboardCleanupConfirmation;
        }
    }

    pub fn confirm_clean_recommendation(&mut self) {
        if let Some(rec_ref) = self.recommendations.get(self.recommendation_selection) {
            let rec = rec_ref.clone();
            match rec.category {
                RecommendationCategory::Docker => {
                    if let Some(_cmd) = &rec.action_command {
                        let _ = self.analyzer.prune();
                    }
                    self.scan_dashboard();
                }
                RecommendationCategory::Log => {
                    let logs: Vec<PathBuf> = self
                        .files
                        .iter()
                        .filter(|f| matches!(f.file_type, FileType::Log))
                        .map(|f| f.path.clone())
                        .collect();

                    for path in logs {
                        let _ = self.cleaner.delete_item(&path);
                    }
                    self.files.retain(|f| !matches!(f.file_type, FileType::Log));
                    self.scan_dashboard();
                }
                RecommendationCategory::Cache => {
                    let items_to_delete: Vec<PathBuf> = self
                        .files
                        .iter()
                        .filter(|f| {
                            matches!(
                                f.file_type,
                                FileType::Cache
                                    | FileType::NpmCache
                                    | FileType::ComposerCache
                                    | FileType::AptCache
                            )
                        })
                        .map(|f| f.path.clone())
                        .collect();

                    for path in items_to_delete {
                        let _ = self.cleaner.delete_item(&path);
                    }
                    self.files.retain(|f| {
                        !matches!(
                            f.file_type,
                            FileType::Cache
                                | FileType::NpmCache
                                | FileType::ComposerCache
                                | FileType::AptCache
                        )
                    });
                    self.scan_dashboard();
                }
                _ => {}
            }
        }
        self.mode = AppMode::Dashboard;
    }

    pub fn cancel_clean(&mut self) {
        self.mode = AppMode::Dashboard;
    }
}
