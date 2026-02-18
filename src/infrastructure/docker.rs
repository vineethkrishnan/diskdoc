use crate::domain::entities::{Recommendation, RecommendationCategory};
use crate::domain::ports::Analyzer;
use anyhow::Result;
use std::process::Command;

pub struct DockerAnalyzerImpl;

impl DockerAnalyzerImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DockerAnalyzerImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer for DockerAnalyzerImpl {
    fn analyze(&self) -> Result<Option<Recommendation>> {
        // Check if docker is running
        let status = Command::new("docker").arg("info").output();

        if status.is_err() || !status.unwrap().status.success() {
            return Ok(None);
        }

        let output = Command::new("docker")
            .args(["system", "df", "--format", "{{.Type}}|{{.Reclaimable}}"])
            .output()?;

        if !output.status.success() {
            return Ok(None);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut total_reclaimable = 0;
        let mut details = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let type_ = parts[0];
                let reclaimable_str = parts[1];

                if let Some(size_str) = reclaimable_str.split(' ').next() {
                    let size = parse_human_size(size_str);
                    if size > 0 {
                        total_reclaimable += size;
                        details.push(format!("{}: {}", type_, reclaimable_str));
                    }
                }
            }
        }

        if total_reclaimable > 0 {
            Ok(Some(Recommendation {
                category: RecommendationCategory::Docker,
                description: format!("Docker Cleanup: {}", details.join(", ")),
                size: total_reclaimable,
                path: None,
                action_command: Some("docker system prune -f".to_string()),
            }))
        } else {
            Ok(None)
        }
    }

    fn prune(&self) -> Result<()> {
        let _ = Command::new("docker")
            .args(["system", "prune", "-f"])
            .output()?;
        Ok(())
    }
}

fn parse_human_size(s: &str) -> u64 {
    let s = s.trim();
    if s.is_empty() {
        return 0;
    }

    let (num_str, unit) = if let Some(stripped) = s.strip_suffix("GB") {
        (stripped, 1024 * 1024 * 1024)
    } else if let Some(stripped) = s.strip_suffix("MB") {
        (stripped, 1024 * 1024)
    } else if let Some(stripped) = s.strip_suffix("KB") {
        (stripped, 1024)
    } else if let Some(stripped) = s.strip_suffix("B") {
        (stripped, 1)
    } else {
        (s, 1)
    };

    if let Ok(num) = num_str.parse::<f64>() {
        (num * unit as f64) as u64
    } else {
        0
    }
}
