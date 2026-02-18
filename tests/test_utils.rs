use anyhow::Result;
use diskdoc::domain::entities::Recommendation;
use diskdoc::domain::ports::{Analyzer, Cleaner};
use std::path::Path;

pub struct MockCleaner;
impl MockCleaner {
    pub fn new() -> Self {
        Self
    }
}
impl Default for MockCleaner {
    fn default() -> Self {
        Self::new()
    }
}
impl Cleaner for MockCleaner {
    fn delete_item(&self, _path: &Path) -> Result<u64> {
        Ok(100)
    }
}

pub struct MockAnalyzer {
    pub recommendation: Option<Recommendation>,
}
impl MockAnalyzer {
    pub fn new() -> Self {
        Self {
            recommendation: None,
        }
    }
}
impl Default for MockAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
impl Analyzer for MockAnalyzer {
    fn analyze(&self) -> Result<Option<Recommendation>> {
        Ok(self.recommendation.clone())
    }
    fn prune(&self) -> Result<()> {
        Ok(())
    }
}
