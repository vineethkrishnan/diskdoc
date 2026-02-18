use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    Normal,
    Log,
    Cache,
    NpmCache,
    ComposerCache,
    AptCache,
    Docker,
}

#[derive(Debug, Clone)]
pub struct FileStats {
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
    pub modified: SystemTime,
    pub children_count: Option<usize>,
    pub file_type: FileType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecommendationCategory {
    Docker,
    Log,
    Cache,
    Trash,
    Other,
}

#[derive(Debug, Clone)]
pub struct Recommendation {
    pub category: RecommendationCategory,
    pub description: String,
    pub size: u64,
    pub path: Option<PathBuf>,
    pub action_command: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Desc,
    Asc,
}
