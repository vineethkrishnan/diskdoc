use crate::domain::entities::FileType;
use std::path::Path;

pub trait Heuristic {
    fn detect(&self, path: &Path, is_dir: bool) -> Option<FileType>;
}

pub struct LogHeuristic;
impl Heuristic for LogHeuristic {
    fn detect(&self, path: &Path, _is_dir: bool) -> Option<FileType> {
        if let Some(ext) = path.extension() {
            if ext == "log" {
                return Some(FileType::Log);
            }
        }
        // Check for specific log directories??
        if path.to_string_lossy().contains("/var/log") {
            return Some(FileType::Log);
        }
        None
    }
}

pub struct CacheHeuristic;
impl Heuristic for CacheHeuristic {
    fn detect(&self, path: &Path, _is_dir: bool) -> Option<FileType> {
        let path_str = path.to_string_lossy();
        // Common cache directories
        if path_str.contains("/Library/Caches")
            || path_str.contains("/.cache")
            || path_str.contains("node_modules")
            || path_str.contains("target")
        {
            // Only flag the root of the cache dir? Or all files inside?
            // For now, let's flag everything inside as Cache type for coloring/filtering
            return Some(FileType::Cache);
        }
        None
    }
}

pub struct NpmHeuristic;
impl Heuristic for NpmHeuristic {
    fn detect(&self, path: &Path, _is_dir: bool) -> Option<FileType> {
        let path_str = path.to_string_lossy();
        if path_str.contains(".npm/_cacache") || path_str.contains("node_modules/.cache") {
            return Some(FileType::NpmCache);
        }
        None
    }
}

pub struct ComposerHeuristic;
impl Heuristic for ComposerHeuristic {
    fn detect(&self, path: &Path, _is_dir: bool) -> Option<FileType> {
        let path_str = path.to_string_lossy();
        if path_str.contains(".composer/cache") {
            return Some(FileType::ComposerCache);
        }
        None
    }
}

pub struct AptHeuristic;
impl Heuristic for AptHeuristic {
    fn detect(&self, path: &Path, _is_dir: bool) -> Option<FileType> {
        let path_str = path.to_string_lossy();
        if path_str.contains("/var/cache/apt/archives") {
            return Some(FileType::AptCache);
        }
        None
    }
}

pub struct DockerHeuristic;
impl Heuristic for DockerHeuristic {
    fn detect(&self, path: &Path, _is_dir: bool) -> Option<FileType> {
        let path_str = path.to_string_lossy();
        if path_str.contains("/var/lib/docker") {
            return Some(FileType::Docker);
        }
        // TODO: detecting docker containers/images via socket is harder and requires async/API
        None
    }
}

pub struct HeuristicsEngine {
    heuristics: Vec<Box<dyn Heuristic + Send + Sync>>,
}

impl HeuristicsEngine {
    pub fn new() -> Self {
        Self {
            heuristics: vec![
                Box::new(LogHeuristic),
                Box::new(NpmHeuristic),
                Box::new(ComposerHeuristic),
                Box::new(AptHeuristic),
                Box::new(CacheHeuristic),
                Box::new(DockerHeuristic),
            ],
        }
    }
}

impl Default for HeuristicsEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl HeuristicsEngine {
    pub fn analyze(&self, path: &Path, is_dir: bool) -> FileType {
        for h in &self.heuristics {
            if let Some(t) = h.detect(path, is_dir) {
                return t;
            }
        }
        FileType::Normal
    }
}
